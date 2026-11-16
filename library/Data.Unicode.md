# Data.Unicode

Versioned Unicode classification, simple case mappings, fold cycles, named tables and checked custom ranges.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Unicode`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Unicode.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Names](Data.Unicode.Names.md), [Data.Unicode.Range](Data.Unicode.Range.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode](Data.Unicode.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Names](Data.Unicode.Names.md), [Data.Unicode.Range](Data.Unicode.Range.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Unicode 17.0.0 classification, simple casing, fold cycles and immutable named range tables.
Character APIs accept only Unicode scalars. Numeric table membership includes
all Unicode code points, including surrogates; UTF-8 validity is a separate
operation. Case/fold numeric helpers retain invalid values, and checked custom
tables report malformed bounds, strides, ordering and metadata explicitly.
Built-in classification never materializes or copies the complete tables.

</details>

# Types and constructors

<a id="type-556e69636f646543617365"></a>

## `UnicodeCase`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeCase</span></code></pre>

One of the three simple, single-scalar case mappings.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 69

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeUpper</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeLower</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeTitle</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span></code></pre>

<a id="type-556e69636f6465436173654572726f72"></a>

## `UnicodeCaseError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeCaseError</span></code></pre>

A precise invalid custom-case rule, carrying its zero-based rule index.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 370

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeCaseInvalidBounds</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeCaseInvalidOrder</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeCaseInvalidMapping</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeCaseInvalidAlternating</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnicodeCaseTooManyRules</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span>)</code></pre>

<a id="type-556e69636f64654361736552616e6765"></a>

## `UnicodeCaseRange`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeCaseRange</span></code></pre>

A complete inclusive simple-case rule with signed upper, lower and title deltas.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 345

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkUnicodeCaseRange</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>)))))</code></pre>

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

<a id="type-556e69636f64655370656369616c43617365"></a>

## `UnicodeSpecialCase`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnicodeSpecialCase</span></code></pre>

An immutable locale override set; explicit identity deltas suppress default mappings.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 392

# Values

<a id="value-756e69636f64652d617a6572692d63617365"></a>

## `unicode-azeri-case`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-azeri-case</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span></code></pre>

Azerbaijani simple-case overrides, sharing the immutable Turkic rules.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 403

<a id="value-756e69636f64652d636173652d636f756e74"></a>

## `unicode-case-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-count</span> :: <span class="tok-constructor">U32</span></code></pre>

Number of independent simple-case mapping selectors.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 78

<a id="value-756e69636f64652d636173652d6572726f722d696e646578"></a>

## `unicode-case-error-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-error-index</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the zero-based index of an invalid custom-case rule.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 383

<a id="value-756e69636f64652d636173652d6572726f722d6d657373616765"></a>

## `unicode-case-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a precise checked custom-case construction failure with its rule index.

[Source](/reference-sources/Data.Unicode.cinch) · line 142

<a id="value-756e69636f64652d636173652d696e646578"></a>

## `unicode-case-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-index</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Observe the stable numeric selector associated with one case.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 82

<a id="value-756e69636f64652d636173652d72616e67652d6174"></a>

## `unicode-case-range-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-at</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>))</code></pre>

Observe one standard case rule; out-of-range indices return None.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 572

<a id="value-756e69636f64652d636173652d72616e67652d636f756e74"></a>

## `unicode-case-range-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count complete standard case range records without allocating their observations.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 581

<a id="value-756e69636f64652d636173652d72616e67652d68696768"></a>

## `unicode-case-range-high`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-high</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The upper inclusive source code point of a case rule.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 354

<a id="value-756e69636f64652d636173652d72616e67652d6c6f77"></a>

## `unicode-case-range-low`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-low</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The lower inclusive source code point of a case rule.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 350

<a id="value-756e69636f64652d636173652d72616e67652d6c6f776572"></a>

## `unicode-case-range-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-lower</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

The signed lowercase delta, or the alternating-case sentinel.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 362

<a id="value-756e69636f64652d636173652d72616e67652d7469746c65"></a>

## `unicode-case-range-title`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-title</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

The signed titlecase delta, or the alternating-case sentinel.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 366

<a id="value-756e69636f64652d636173652d72616e67652d7570706572"></a>

## `unicode-case-range-upper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-upper</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

The signed uppercase delta, or the alternating-case sentinel.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 358

<a id="value-756e69636f64652d636173652d72616e676573"></a>

## `unicode-case-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-ranges</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>))</code></pre>

Observe every exact immutable standard case range without exposing backing storage.

[Source](/reference-sources/Data.Unicode.cinch) · line 76

<a id="value-756e69636f64652d63617465676f7279"></a>

## `unicode-category`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Resolve one exact immutable unicode-category name without normalizing punctuation or case.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 24

<a id="value-756e69636f64652d63617465676f72792d616c6961732d6174"></a>

## `unicode-category-alias-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-alias-at</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Observe one sorted alias and its canonical category name; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3459

<a id="value-756e69636f64652d63617465676f72792d616c6961732d636f756e74"></a>

## `unicode-category-alias-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-alias-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Number of immutable category-alias observations.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3455

<a id="value-756e69636f64652d63617465676f72792d616c6961736573"></a>

## `unicode-category-aliases`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-aliases</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Observe the complete immutable category-alias map as sorted key/value pairs.

[Source](/reference-sources/Data.Unicode.cinch) · line 138

<a id="value-756e69636f64652d63617465676f72792d63616e6f6e6963616c"></a>

## `unicode-category-canonical`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-canonical</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Resolve an exact standard category alias to its canonical abbreviated name.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3408

<a id="value-756e69636f64652d63617465676f72792d636f756e74"></a>

## `unicode-category-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count the exact unicode-category catalog keys, including documented aliases.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 219

<a id="value-756e69636f64652d63617465676f72792d6e616d65"></a>

## `unicode-category-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-name</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Read one sorted immutable unicode-category key; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 223

<a id="value-756e69636f64652d63617465676f72792d6e616d6573"></a>

## `unicode-category-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-names</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe every unicode-category key without exposing or copying range-table storage.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 283

<a id="value-756e69636f64652d63617465676f72792d6f662d636f6465"></a>

## `unicode-category-of-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-of-code</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe the primary category of any numeric Unicode code point; invalid or unassigned scripts return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3506

<a id="value-756e69636f64652d63617465676f72792d7265736f6c7665"></a>

## `unicode-category-resolve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-category-resolve</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Resolve a canonical category name first, then an exact standard category alias.

[Source](/reference-sources/Data.Unicode.cinch) · line 62

<a id="value-756e69636f64652d657175616c2d666f6c64"></a>

## `unicode-equal-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-equal-fold</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test whether two scalars belong to the same simple-fold cycle.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 210

<a id="value-756e69636f64652d657175616c2d666f6c642d636f6465"></a>

## `unicode-equal-fold-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-equal-fold-code</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test numeric simple-fold equivalence; identical invalid inputs compare equal.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 204

<a id="value-756e69636f64652d666f6c642d63617465676f7279"></a>

## `unicode-fold-category`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-category</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Resolve one exact immutable unicode-fold-category name without normalizing punctuation or case.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1649

<a id="value-756e69636f64652d666f6c642d63617465676f72792d636f756e74"></a>

## `unicode-fold-category-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-category-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count the exact unicode-fold-category catalog keys, including documented aliases.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1684

<a id="value-756e69636f64652d666f6c642d63617465676f72792d6e616d65"></a>

## `unicode-fold-category-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-category-name</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Read one sorted immutable unicode-fold-category key; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1688

<a id="value-756e69636f64652d666f6c642d63617465676f72792d6e616d6573"></a>

## `unicode-fold-category-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-category-names</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe every unicode-fold-category key without exposing or copying range-table storage.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1716

<a id="value-756e69636f64652d666f6c642d736372697074"></a>

## `unicode-fold-script`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-script</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Resolve one exact immutable unicode-fold-script name without normalizing punctuation or case.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1720

<a id="value-756e69636f64652d666f6c642d7363726970742d636f756e74"></a>

## `unicode-fold-script-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-script-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count the exact unicode-fold-script catalog keys, including documented aliases.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1740

<a id="value-756e69636f64652d666f6c642d7363726970742d6e616d65"></a>

## `unicode-fold-script-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-script-name</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Read one sorted immutable unicode-fold-script key; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1744

<a id="value-756e69636f64652d666f6c642d7363726970742d6e616d6573"></a>

## `unicode-fold-script-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-fold-script-names</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe every unicode-fold-script key without exposing or copying range-table storage.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1769

<a id="value-756e69636f64652d677261706869632d72616e676573"></a>

## `unicode-graphic-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-graphic-ranges</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>)</code></pre>

Immutable category tables defining graphic characters: letters, marks, numbers, punctuation, symbols and space separators.

[Source](/reference-sources/Data.Unicode.cinch) · line 122

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

<a id="value-756e69636f64652d69732d636f6e74726f6c"></a>

## `unicode-is-control`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-control</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode control class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 218

<a id="value-756e69636f64652d69732d636f6e74726f6c2d636f6465"></a>

## `unicode-is-control-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-control-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode control class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 214

<a id="value-756e69636f64652d69732d6469676974"></a>

## `unicode-is-digit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-digit</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode digit class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 226

<a id="value-756e69636f64652d69732d64696769742d636f6465"></a>

## `unicode-is-digit-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-digit-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode digit class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 222

<a id="value-756e69636f64652d69732d67726170686963"></a>

## `unicode-is-graphic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-graphic</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode graphic class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 298

<a id="value-756e69636f64652d69732d677261706869632d636f6465"></a>

## `unicode-is-graphic-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-graphic-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode graphic class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 294

<a id="value-756e69636f64652d69732d6c6574746572"></a>

## `unicode-is-letter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-letter</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode letter class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 234

<a id="value-756e69636f64652d69732d6c65747465722d636f6465"></a>

## `unicode-is-letter-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-letter-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode letter class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 230

<a id="value-756e69636f64652d69732d6c6f776572"></a>

## `unicode-is-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-lower</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode lower class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 242

<a id="value-756e69636f64652d69732d6c6f7765722d636f6465"></a>

## `unicode-is-lower-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-lower-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode lower class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 238

<a id="value-756e69636f64652d69732d6d61726b"></a>

## `unicode-is-mark`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-mark</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode mark class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 266

<a id="value-756e69636f64652d69732d6d61726b2d636f6465"></a>

## `unicode-is-mark-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-mark-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode mark class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 262

<a id="value-756e69636f64652d69732d6e756d626572"></a>

## `unicode-is-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-number</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode number class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 274

<a id="value-756e69636f64652d69732d6e756d6265722d636f6465"></a>

## `unicode-is-number-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-number-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode number class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 270

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

<a id="value-756e69636f64652d69732d7072696e74"></a>

## `unicode-is-print`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-print</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode print class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 306

<a id="value-756e69636f64652d69732d7072696e742d636f6465"></a>

## `unicode-is-print-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-print-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode print class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 302

<a id="value-756e69636f64652d69732d70756e6374"></a>

## `unicode-is-punct`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-punct</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode punct class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 282

<a id="value-756e69636f64652d69732d70756e63742d636f6465"></a>

## `unicode-is-punct-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-punct-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode punct class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 278

<a id="value-756e69636f64652d69732d7363616c61722d636f6465"></a>

## `unicode-is-scalar-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-scalar-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test a signed numeric value for Unicode scalar validity, excluding surrogates.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 86

<a id="value-756e69636f64652d69732d7370616365"></a>

## `unicode-is-space`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-space</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test whether one scalar is Unicode whitespace.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 317

<a id="value-756e69636f64652d69732d73706163652d636f6465"></a>

## `unicode-is-space-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-space-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test Unicode whitespace, including Latin-1 separators and the full White_Space property.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 310

<a id="value-756e69636f64652d69732d73796d626f6c"></a>

## `unicode-is-symbol`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-symbol</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode symbol class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 290

<a id="value-756e69636f64652d69732d73796d626f6c2d636f6465"></a>

## `unicode-is-symbol-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-symbol-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode symbol class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 286

<a id="value-756e69636f64652d69732d7469746c65"></a>

## `unicode-is-title`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-title</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode title class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 258

<a id="value-756e69636f64652d69732d7469746c652d636f6465"></a>

## `unicode-is-title-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-title-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode title class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 254

<a id="value-756e69636f64652d69732d7570706572"></a>

## `unicode-is-upper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-upper</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode upper class of one valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 250

<a id="value-756e69636f64652d69732d75707065722d636f6465"></a>

## `unicode-is-upper-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-is-upper-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test the Unicode upper class for a signed code point, rejecting numbers outside the Unicode domain.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 246

<a id="value-756e69636f64652d6c6174696e2d6f6666736574"></a>

## `unicode-latin-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-latin-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the exact original built-in Latin offset, or the calculated custom prefix count.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 119

<a id="value-756e69636f64652d6d61782d6173636969"></a>

## `unicode-max-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-max-ascii</span> :: <span class="tok-constructor">U32</span></code></pre>

Highest ASCII code point.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 53

<a id="value-756e69636f64652d6d61782d636f6465706f696e74"></a>

## `unicode-max-codepoint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-max-codepoint</span> :: <span class="tok-constructor">U32</span></code></pre>

Highest Unicode code point; this bound alone does not exclude surrogates.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 49

<a id="value-756e69636f64652d6d61782d6c6174696e31"></a>

## `unicode-max-latin1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-max-latin1</span> :: <span class="tok-constructor">U32</span></code></pre>

Highest Latin-1 code point.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 57

<a id="value-756e69636f64652d6d61782d72756e65"></a>

## `unicode-max-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-max-rune</span> :: <span class="tok-constructor">Char</span></code></pre>

The greatest valid Unicode scalar value, including its noncharacter status.

[Source](/reference-sources/Data.Unicode.cinch) · line 58

<a id="value-756e69636f64652d7072696e742d72616e676573"></a>

## `unicode-print-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-print-ranges</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>)</code></pre>

Immutable category tables used by print classification; the separate ASCII-space rule remains observable.

[Source](/reference-sources/Data.Unicode.cinch) · line 126

<a id="value-756e69636f64652d70726f7065727479"></a>

## `unicode-property`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-property</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Resolve one exact immutable unicode-property name without normalizing punctuation or case.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1380

<a id="value-756e69636f64652d70726f70657274792d636f756e74"></a>

## `unicode-property-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-property-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count the exact unicode-property catalog keys, including documented aliases.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1580

<a id="value-756e69636f64652d70726f70657274792d6e616d65"></a>

## `unicode-property-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-property-name</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Read one sorted immutable unicode-property key; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1584

<a id="value-756e69636f64652d70726f70657274792d6e616d6573"></a>

## `unicode-property-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-property-names</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe every unicode-property key without exposing or copying range-table storage.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1645

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

<a id="value-756e69636f64652d72616e67652d6572726f722d6d657373616765"></a>

## `unicode-range-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a precise checked custom-range construction failure without reading invalid backing storage.

[Source](/reference-sources/Data.Unicode.cinch) · line 148

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

<a id="value-756e69636f64652d72616e676531362d72616e676573"></a>

## `unicode-range16-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range16-ranges</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange16</span>))</code></pre>

Observe every exact 16-bit range record in its original order.

[Source](/reference-sources/Data.Unicode.cinch) · line 100

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

<a id="value-756e69636f64652d72616e676533322d72616e676573"></a>

## `unicode-range32-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range32-ranges</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span>))</code></pre>

Observe every exact supplementary-plane range record in its original order.

[Source](/reference-sources/Data.Unicode.cinch) · line 112

<a id="value-756e69636f64652d72616e676533322d737472696465"></a>

## `unicode-range32-stride`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-range32-stride</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRange32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The exact original 32-bit stride, including values larger than the Unicode domain.

[Source](/reference-sources/Data.Unicode.Range.cinch) · line 57

<a id="value-756e69636f64652d7265706c6163656d656e74"></a>

## `unicode-replacement`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-replacement</span> :: <span class="tok-constructor">Char</span></code></pre>

The Unicode replacement scalar U+FFFD.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 61

<a id="value-756e69636f64652d736372697074"></a>

## `unicode-script`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-script</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Resolve one exact immutable unicode-script name without normalizing punctuation or case.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 287

<a id="value-756e69636f64652d7363726970742d636f756e74"></a>

## `unicode-script-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-script-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count the exact unicode-script catalog keys, including documented aliases.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1162

<a id="value-756e69636f64652d7363726970742d6e616d65"></a>

## `unicode-script-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-script-name</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Read one sorted immutable unicode-script key; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1166

<a id="value-756e69636f64652d7363726970742d6e616d6573"></a>

## `unicode-script-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-script-names</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe every unicode-script key without exposing or copying range-table storage.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1376

<a id="value-756e69636f64652d7363726970742d6f662d636f6465"></a>

## `unicode-script-of-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-script-of-code</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe the primary script of any numeric Unicode code point; invalid or unassigned scripts return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3555

<a id="value-756e69636f64652d73696d706c652d666f6c64"></a>

## `unicode-simple-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-simple-fold</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

Return the next scalar in its simple-fold cycle without normalization or expansion.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 194

<a id="value-756e69636f64652d73696d706c652d666f6c642d636f6465"></a>

## `unicode-simple-fold-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-simple-fold-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

Return the numeric-order successor in a simple-fold cycle; invalid numbers remain unchanged.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 184

<a id="value-756e69636f64652d7370656369616c2d616666656374732d6173636969"></a>

## `unicode-special-affects-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-affects-ascii</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether any override can affect ASCII, allowing bulk default ASCII mapping when false.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 609

<a id="value-756e69636f64652d7370656369616c2d63617365"></a>

## `unicode-special-case`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-case</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span>))</code></pre>

Construct scalar-preserving ordered custom rules, with identity rules overriding default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 451

<a id="value-756e69636f64652d7370656369616c2d636173652d636f756e74"></a>

## `unicode-special-case-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-case-count</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Count immutable override records without materializing their observations.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 553

<a id="value-756e69636f64652d7370656369616c2d636173652d72616e67652d6174"></a>

## `unicode-special-case-range-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-case-range-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>)))</code></pre>

Observe one override record; out-of-range indices return None.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 566

<a id="value-756e69636f64652d7370656369616c2d636173652d72616e676573"></a>

## `unicode-special-case-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-case-ranges</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>))</code></pre>

Observe every immutable override rule without exposing its private byte storage.

[Source](/reference-sources/Data.Unicode.cinch) · line 88

<a id="value-756e69636f64652d7370656369616c2d746f"></a>

## `unicode-special-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)))</code></pre>

Apply one scalar-preserving locale override mapping to a valid character.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 549

<a id="value-756e69636f64652d7370656369616c2d746f2d6c6f776572"></a>

## `unicode-special-to-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-lower</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>))</code></pre>

Apply scalar-preserving locale lowercase overrides before default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 597

<a id="value-756e69636f64652d7370656369616c2d746f2d6c6f7765722d636f6465"></a>

## `unicode-special-to-lower-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-lower-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

Apply locale lowercase overrides to a numeric code point, preserving invalid inputs.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 593

<a id="value-756e69636f64652d7370656369616c2d746f2d7469746c65"></a>

## `unicode-special-to-title`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-title</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>))</code></pre>

Apply scalar-preserving locale titlecase overrides before default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 605

<a id="value-756e69636f64652d7370656369616c2d746f2d7469746c652d636f6465"></a>

## `unicode-special-to-title-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-title-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

Apply locale titlecase overrides to a numeric code point, preserving invalid inputs.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 601

<a id="value-756e69636f64652d7370656369616c2d746f2d7570706572"></a>

## `unicode-special-to-upper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-upper</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>))</code></pre>

Apply scalar-preserving locale uppercase overrides before default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 589

<a id="value-756e69636f64652d7370656369616c2d746f2d75707065722d636f6465"></a>

## `unicode-special-to-upper-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-upper-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

Apply locale uppercase overrides to a numeric code point, preserving invalid inputs.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 585

<a id="value-756e69636f64652d7461626c65"></a>

## `unicode-table`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-table</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Range</span>.<span class="tok-constructor">UnicodeRangeTable</span>))</code></pre>

Resolve one exact immutable unicode-table name without normalizing punctuation or case.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 1773

<a id="value-756e69636f64652d7461626c652d636f756e74"></a>

## `unicode-table-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-table-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count the exact unicode-table catalog keys, including documented aliases.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3088

<a id="value-756e69636f64652d7461626c652d6e616d65"></a>

## `unicode-table-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-table-name</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Read one sorted immutable unicode-table key; invalid indices return None.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3092

<a id="value-756e69636f64652d7461626c652d6e616d6573"></a>

## `unicode-table-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-table-names</span> :: (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Observe every unicode-table key without exposing or copying range-table storage.

[Source](/reference-sources/Data.Unicode.Names.cinch) · line 3404

<a id="value-756e69636f64652d746f"></a>

## `unicode-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>))</code></pre>

Apply one simple case mapping to a valid scalar.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 169

<a id="value-756e69636f64652d746f2d636f6465"></a>

## `unicode-to-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to-code</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

Map a signed selector and numeric code point; an invalid selector returns U+FFFD.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 165

<a id="value-756e69636f64652d746f2d6c6f776572"></a>

## `unicode-to-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to-lower</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

Return the simple single-scalar lowercase mapping.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 333

<a id="value-756e69636f64652d746f2d6c6f7765722d636f6465"></a>

## `unicode-to-lower-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to-lower-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

Return the simple lowercase mapping of a numeric code point; invalid inputs remain unchanged.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 329

<a id="value-756e69636f64652d746f2d7469746c65"></a>

## `unicode-to-title`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to-title</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

Return the simple single-scalar titlecase mapping.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 341

<a id="value-756e69636f64652d746f2d7469746c652d636f6465"></a>

## `unicode-to-title-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to-title-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

Return the simple titlecase mapping of a numeric code point; invalid inputs remain unchanged.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 337

<a id="value-756e69636f64652d746f2d7570706572"></a>

## `unicode-to-upper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to-upper</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

Return the simple single-scalar uppercase mapping.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 325

<a id="value-756e69636f64652d746f2d75707065722d636f6465"></a>

## `unicode-to-upper-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-to-upper-code</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

Return the simple uppercase mapping of a numeric code point; invalid inputs remain unchanged.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 321

<a id="value-756e69636f64652d7475726b6973682d63617365"></a>

## `unicode-turkish-case`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-turkish-case</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span></code></pre>

Turkish simple-case overrides, taking precedence over default mappings.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 399

<a id="value-756e69636f64652d75707065722d6c6f776572"></a>

## `unicode-upper-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-upper-lower</span> :: <span class="tok-constructor">I32</span></code></pre>

Sentinel delta used by alternating uppercase/lowercase range observations.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 65

<a id="value-756e69636f64652d76657273696f6e"></a>

## `unicode-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-version</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

Exact edition of every built-in classification, script, property and simple mapping.

[Source](/reference-sources/Data.Unicode.cinch) · line 54

<a id="value-756e69636f64652d76657273696f6e2d6d616a6f72"></a>

## `unicode-version-major`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-version-major</span> :: <span class="tok-constructor">U32</span></code></pre>

Major edition of the immutable classification and mapping data.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 37

<a id="value-756e69636f64652d76657273696f6e2d6d696e6f72"></a>

## `unicode-version-minor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-version-minor</span> :: <span class="tok-constructor">U32</span></code></pre>

Minor edition of the immutable classification and mapping data.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 41

<a id="value-756e69636f64652d76657273696f6e2d7061746368"></a>

## `unicode-version-patch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-version-patch</span> :: <span class="tok-constructor">U32</span></code></pre>

Patch edition of the immutable classification and mapping data.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 45

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Unicode.cinch`. Source SHA-256: `fba58333635e0995c1ae77eb1947275c7198a557147a95333f8ef52d981da2c0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
