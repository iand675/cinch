# Data.Unicode.Kernel

Unicode scalar and numeric classification, simple case mappings, fold cycles and checked locale overrides.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Unicode.Kernel`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Unicode.Kernel.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Unicode 17.0.0 scalar classification, simple casing and simple-fold cycles.
This numeric core has no direct String dependency. Character APIs operate on
scalars; signed code-point APIs retain invalid numeric casing/folding inputs.
Classification rejects out-of-domain numbers while permitting surrogate
code points in numeric table observations. Case conversion returns one scalar;
simple folding walks canonical numeric-order equivalence cycles.

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

<a id="value-756e69636f64652d636173652d696e646578"></a>

## `unicode-case-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-index</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Observe the stable numeric selector associated with one case.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 82

<a id="value-756e69636f64652d636173652d72616e67652d6174"></a>

## `unicode-case-range-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-at</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>))</code></pre>

Observe one standard case rule; out-of-range indices return None.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 560

<a id="value-756e69636f64652d636173652d72616e67652d636f756e74"></a>

## `unicode-case-range-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-case-range-count</span> :: <span class="tok-constructor">Nat</span></code></pre>

Count complete standard case range records without allocating their observations.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 569

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

<a id="value-756e69636f64652d7265706c6163656d656e74"></a>

## `unicode-replacement`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-replacement</span> :: <span class="tok-constructor">Char</span></code></pre>

The Unicode replacement scalar U+FFFD.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 61

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

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 597

<a id="value-756e69636f64652d7370656369616c2d63617365"></a>

## `unicode-special-case`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-case</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span>))</code></pre>

Construct scalar-preserving ordered custom rules, with identity rules overriding default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 451

<a id="value-756e69636f64652d7370656369616c2d636173652d636f756e74"></a>

## `unicode-special-case-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-case-count</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Count immutable override records without materializing their observations.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 541

<a id="value-756e69636f64652d7370656369616c2d636173652d72616e67652d6174"></a>

## `unicode-special-case-range-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-case-range-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCaseRange</span>)))</code></pre>

Observe one override record; out-of-range indices return None.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 554

<a id="value-756e69636f64652d7370656369616c2d746f"></a>

## `unicode-special-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)))</code></pre>

Apply one scalar-preserving locale override mapping to a valid character.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 537

<a id="value-756e69636f64652d7370656369616c2d746f2d6c6f776572"></a>

## `unicode-special-to-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-lower</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>))</code></pre>

Apply scalar-preserving locale lowercase overrides before default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 585

<a id="value-756e69636f64652d7370656369616c2d746f2d6c6f7765722d636f6465"></a>

## `unicode-special-to-lower-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-lower-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

Apply locale lowercase overrides to a numeric code point, preserving invalid inputs.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 581

<a id="value-756e69636f64652d7370656369616c2d746f2d7469746c65"></a>

## `unicode-special-to-title`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-title</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>))</code></pre>

Apply scalar-preserving locale titlecase overrides before default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 593

<a id="value-756e69636f64652d7370656369616c2d746f2d7469746c652d636f6465"></a>

## `unicode-special-to-title-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-title-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

Apply locale titlecase overrides to a numeric code point, preserving invalid inputs.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 589

<a id="value-756e69636f64652d7370656369616c2d746f2d7570706572"></a>

## `unicode-special-to-upper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-upper</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>))</code></pre>

Apply scalar-preserving locale uppercase overrides before default casing.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 577

<a id="value-756e69636f64652d7370656369616c2d746f2d75707065722d636f6465"></a>

## `unicode-special-to-upper-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unicode-special-to-upper-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Unicode</span>.<span class="tok-constructor">Kernel</span>.<span class="tok-constructor">UnicodeSpecialCase</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

Apply locale uppercase overrides to a numeric code point, preserving invalid inputs.

[Source](/reference-sources/Data.Unicode.Kernel.cinch) · line 573

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

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Unicode/Kernel.cinch`. Source SHA-256: `ea234b1282e1f6e3e6bcee9b9d91fccb3d2bd74c9301c5c52bef6b9f0c0522d2`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
