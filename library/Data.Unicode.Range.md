# Data.Unicode.Range

Immutable Unicode range tables with checked construction, exact membership and copied observations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Unicode.Range`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Unicode.Range.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Range](Data.Unicode.Range.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable strided Unicode range tables with exact indexed observations.
Numeric membership includes surrogate code points; scalar construction and
UTF-8 validity remain separate operations. Built-in queries read the compact
class database without copying a table. Custom tables validate ordered bounds,
nonzero strides and Latin-1 metadata before creating private owned storage.

</details>

# Types and constructors

<a id="type-556e69636f646552616e67653136"></a>

## `UnicodeRange16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeRange16</span></code></pre>

An inclusive 16-bit strided code-point interval, which may include surrogates.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 27

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkUnicodeRange16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span>)))</code></pre>

<a id="type-556e69636f646552616e67653332"></a>

## `UnicodeRange32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeRange32</span></code></pre>

An inclusive supplementary-plane strided code-point interval.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 32

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkUnicodeRange32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span>)))</code></pre>

<a id="type-556e69636f646552616e67654572726f72"></a>

## `UnicodeRangeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeRangeError</span></code></pre>

A precise invalid custom interval or metadata observation.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 61

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeRangeInvalidBounds</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeRangeZeroStride</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeRangeInvalidOrder</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeRangeInvalidLatinOffset</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeRangeTooManyRecords</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span>))</code></pre>

<a id="type-556e69636f646552616e67655461626c65"></a>

## `UnicodeRangeTable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeRangeTable</span></code></pre>

An immutable built-in table or private copied custom interval storage.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 92

# Values

<a id="value-756e69636f64652d6275696c74696e2d72616e67652d7461626c65"></a>

## `unicode-builtin-range-table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-builtin-range-table</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Select one immutable built-in identity, rejecting out-of-range internal identifiers.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 99

<a id="value-756e69636f64652d696e"></a>

## `unicode-in`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-in</span> :: (<span class="tok-constructor">Char</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>) -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test scalar membership in any supplied immutable table.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 306

<a id="value-756e69636f64652d696e2d636f6465"></a>

## `unicode-in-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-in-code</span> :: (<span class="tok-constructor">I32</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>) -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test numeric membership in any supplied immutable table, retaining invalid-input policy.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 302

<a id="value-756e69636f64652d69732d6f6e652d6f66"></a>

## `unicode-is-one-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-one-of</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>) -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test a table list before a scalar, with the same union semantics as unicode-in.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 314

<a id="value-756e69636f64652d69732d6f6e652d6f662d636f6465"></a>

## `unicode-is-one-of-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-one-of-code</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>) -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test a table list before a numeric code point, with the same union semantics as unicode-in-code.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 310

<a id="value-756e69636f64652d6c6174696e2d6f6666736574"></a>

## `unicode-latin-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-latin-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the exact original built-in Latin offset, or the calculated custom prefix count.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 119

<a id="value-756e69636f64652d72616e67652d636f6e7461696e73"></a>

## `unicode-range-contains`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-contains</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test the membership of one valid scalar without copying or exposing a backing table.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 298

<a id="value-756e69636f64652d72616e67652d636f6e7461696e732d636f6465"></a>

## `unicode-range-contains-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-contains-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test numeric membership, including surrogates and excluding negative or out-of-domain values.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 287

<a id="value-756e69636f64652d72616e67652d6572726f722d696e646578"></a>

## `unicode-range-error-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-error-index</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the zero-based invalid interval index; metadata-only failures return zero.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 74

<a id="value-756e69636f64652d72616e67652d6572726f722d69732d77696465"></a>

## `unicode-range-error-is-wide`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-error-is-wide</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Identify an invalid interval in the supplementary-plane half; metadata-only failures return false.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 83

<a id="value-756e69636f64652d72616e67652d7461626c65"></a>

## `unicode-range-table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-table</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>)))</code></pre>

Construct a validated custom code-point table, including legitimate surrogate intervals.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 187

<a id="value-756e69636f64652d72616e67652d7461626c652d636c6f6e65"></a>

## `unicode-range-table-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-table-clone</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>)</code></pre>

Retain an independent immutable table value without copying private storage.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 103

<a id="value-756e69636f64652d72616e67652d7461626c652d776974682d6f6666736574"></a>

## `unicode-range-table-with-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-table-with-offset</span> :: (<span class="tok-constructor">I32</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))))</code></pre>

Construct a custom table only when the explicit Latin prefix matches its validated metadata.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 193

<a id="value-756e69636f64652d72616e676531362d6174"></a>

## `unicode-range16-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range16-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span>)))</code></pre>

Observe a 16-bit record by zero-based index; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 215

<a id="value-756e69636f64652d72616e676531362d636f756e74"></a>

## `unicode-range16-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range16-count</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the original number of 16-bit records without materializing the table.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 107

<a id="value-756e69636f64652d72616e676531362d68696768"></a>

## `unicode-range16-high`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range16-high</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

The upper inclusive 16-bit bound, whether or not the stride lands on it.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 41

<a id="value-756e69636f64652d72616e676531362d6c6f77"></a>

## `unicode-range16-low`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range16-low</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

The lower inclusive 16-bit bound.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 37

<a id="value-756e69636f64652d72616e676531362d737472696465"></a>

## `unicode-range16-stride`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range16-stride</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

The exact original 16-bit stride.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 45

<a id="value-756e69636f64652d72616e676533322d6174"></a>

## `unicode-range32-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range32-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span>)))</code></pre>

Observe a supplementary-plane record by zero-based index; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 222

<a id="value-756e69636f64652d72616e676533322d636f756e74"></a>

## `unicode-range32-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range32-count</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the original number of supplementary-plane records without materializing them.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 113

<a id="value-756e69636f64652d72616e676533322d68696768"></a>

## `unicode-range32-high`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range32-high</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The upper inclusive supplementary-plane bound, whether or not the stride lands on it.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 53

<a id="value-756e69636f64652d72616e676533322d6c6f77"></a>

## `unicode-range32-low`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range32-low</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The lower inclusive supplementary-plane bound.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 49

<a id="value-756e69636f64652d72616e676533322d737472696465"></a>

## `unicode-range32-stride`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range32-stride</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The exact original 32-bit stride, including values larger than the Unicode domain.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 57

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Unicode/Range.cinch`. Source SHA-256: `36f645bc334d75474d5dd70f1aa680d330a97476a89a41ee872d880620c036c8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
