# Data.SuffixArray

Immutable O(n log n) suffix arrays over raw bytes with binary-search lookup, regex search and checked versioned binary serialization.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.SuffixArray`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.SuffixArray.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Regex](Text.Regex.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.SuffixArray](Data.SuffixArray.md), [Text.Regex](Text.Regex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-5375666669784172726179"></a>

## `SuffixArray`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SuffixArray</span></code></pre>

An immutable suffix array over a private snapshot of the indexed bytes
together with its packed sorted suffix offsets.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 39

<a id="type-53756666697841727261794572726f72"></a>

## `SuffixArrayError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SuffixArrayError</span></code></pre>

Reasons why decoding a serialized suffix array can fail.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 44

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SuffixArrayInvalidHeader</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArrayError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SuffixArrayTruncated</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArrayError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SuffixArrayCorruptIndex</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArrayError</span></code></pre>

# Values

<a id="value-7375666669782d61727261792d6174"></a>

## `suffix-array-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Read the suffix starting offset at position pos in the sorted suffix array.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 77

<a id="value-7375666669782d61727261792d6279746573"></a>

## `suffix-array-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh copy of the indexed bytes so the caller cannot mutate internal storage.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 67

<a id="value-7375666669782d61727261792d636f756e74"></a>

## `suffix-array-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-count</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Count all occurrences of pattern in the indexed bytes in O(log(n) * len(pattern)) time.
An empty pattern returns zero.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 85

<a id="value-7375666669782d61727261792d6465636f6465"></a>

## `suffix-array-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArrayError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span>))</code></pre>

Decode and validate a "CSA1" binary suffix array representation.
Rejects malformed headers, truncated payloads, trailing bytes, out-of-range offsets,
duplicate offsets, and unsorted suffix tables.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 151

<a id="value-7375666669782d61727261792d656e636f6465"></a>

## `suffix-array-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-encode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a suffix array into the versioned "CSA1" binary representation containing
the 16-byte header, raw indexed bytes, and packed little-endian suffix offsets.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 135

<a id="value-7375666669782d61727261792d6572726f722d6d657373616765"></a>

## `suffix-array-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArrayError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Human-readable explanation of a suffix array decode error.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 53

<a id="value-7375666669782d61727261792d66696e642d616c6c2d7265676578"></a>

## `suffix-array-find-all-regex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-find-all-regex</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Regex</span>.<span class="tok-constructor">Regex</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Find up to limit non-overlapping matches of a compiled regular expression as (start, end)
byte offset pairs in ascending order. A negative limit returns all matches; zero returns Nil.
Returns Nil when the indexed bytes are not valid UTF-8.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 123

<a id="value-7375666669782d61727261792d6c656e"></a>

## `suffix-array-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-len</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Number of indexed bytes (and suffixes) in the suffix array.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 72

<a id="value-7375666669782d61727261792d6c6f6f6b7570"></a>

## `suffix-array-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-lookup</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Return at most limit starting offsets where pat occurs in the indexed bytes.
A negative limit returns all matches; zero or an empty pattern returns Nil.
Offsets are returned in suffix-array order in O(log(n) * len(pat) + k) time.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 99

<a id="value-7375666669782d61727261792d6c6f6f6b75702d736f72746564"></a>

## `suffix-array-lookup-sorted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-lookup-sorted</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Return at most limit starting offsets where pat occurs, sorted in ascending offset order.
A negative limit returns all matches in ascending document order; zero or an empty pattern returns Nil.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 116

<a id="value-7375666669782d61727261792d6e6577"></a>

## `suffix-array-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">suffix-array-new</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">SuffixArray</span>.<span class="tok-constructor">SuffixArray</span>)</code></pre>

Build an immutable suffix array over a fresh snapshot copy of the input bytes.

[Source](/reference-sources/Data.SuffixArray.cinch) · line 60

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/SuffixArray.cinch`. Source SHA-256: `15067cb5228dd78947799c4a31627d9bc7aa3f3c3903daca6793268c6f837d3c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
