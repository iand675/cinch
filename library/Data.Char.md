# Data.Char

Unicode scalar values and checked character/code conversions, with range information for safe use.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Char</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Char/source/">View source</a><a href="/packages/search/?q=%2BData.Char">Search module</a><a href="/reference-sources/Data.Char.cinch" download="Data.Char.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Hash](Data.Hash.md), [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Char: Unicode scalar values.

`Char` is a core runtime type (primitive types specification
section 1): one scalar value. `u32-to-char` rejects values outside
the scalar range (surrogates and above 0x10FFFF) as `None`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

<details class="doc-synopsis"><summary>Synopsis (21 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-char-code">char-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">char-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 1114111 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-char-from-code">char-from-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">char-from-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-char-to-u32">char-to-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">char-to-u32</strong> :: (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (0 &lt;= <span class="tok-variable">cinch-result</span> &amp;&amp; <span class="tok-variable">cinch-result</span> &lt;= 1114111 &amp;&amp; <span class="tok-variable">cinch-result</span> == <span class="tok-variable">c</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-digit-value">digit-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">digit-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-eq-char">eq-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">eq-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ge-char">ge-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ge-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-gt-char">gt-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gt-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-hex-digit-value">hex-digit-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-digit-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-is-ascii-alpha">is-ascii-alpha</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ascii-alpha</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-ascii-alpha-numeric">is-ascii-alpha-numeric</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ascii-alpha-numeric</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-ascii-digit">is-ascii-digit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ascii-digit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-ascii-hex-digit">is-ascii-hex-digit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ascii-hex-digit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-ascii-lower">is-ascii-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ascii-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-ascii-space">is-ascii-space</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ascii-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-ascii-upper">is-ascii-upper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-ascii-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-le-char">le-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">le-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-lt-char">lt-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lt-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-ne-char">ne-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ne-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-to-lower-ascii">to-lower-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-lower-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-to-upper-ascii">to-upper-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-upper-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-u32-to-char">u32-to-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">u32-to-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-char-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">char-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 1114111 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-char-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

The scalar value as a `Nat`: the conversion the text and data
homes all spelled out by hand before this was its home. A scalar
value is at most 0x10FFFF, and the result says so.

</div>
</article>

<article class="doc-entry" id="v-char-from-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">char-from-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-char-from-code" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

The inverse, partial exactly as `u32-to-char` is: a value outside
the scalar range (a surrogate, or above 0x10FFFF) is `None`. The
range test comes first, so a value past `U32` is `None` rather than
a wrapped scalar.

</div>
</article>

<article class="doc-entry" id="v-char-to-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">char-to-u32</strong> :: (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (0 &lt;= <span class="tok-variable">cinch-result</span> &amp;&amp; <span class="tok-variable">cinch-result</span> &lt;= 1114111 &amp;&amp; <span class="tok-variable">cinch-result</span> == <span class="tok-variable">c</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-char-to-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Primitive types specification, section 6: Char values are Unicode
scalar values, in code point order. `u32-to-char` rejects surrogates
and values above 0x10FFFF.

</div>
</article>

<article class="doc-entry" id="v-digit-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">digit-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-digit-value" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

`Some 0..9` for the ASCII digits, `None` otherwise (the caller
has already decided what a non-digit means).

</div>
</article>

<article class="doc-entry" id="v-eq-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">eq-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-eq-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L27" title="View source at line 27">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ge-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ge-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ge-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L32" title="View source at line 32">Source</a></span></div>
</article>

<article class="doc-entry" id="v-gt-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gt-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-gt-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L31" title="View source at line 31">Source</a></span></div>
</article>

<article class="doc-entry" id="v-hex-digit-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-digit-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hex-digit-value" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

`Some 0..15` for `0`-`9`, `a`-`f` and `A`-`F`; `None` otherwise.

</div>
</article>

<article class="doc-entry" id="v-is-ascii-alpha">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ascii-alpha</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ascii-alpha" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L102" title="View source at line 102">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-ascii-alpha-numeric">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ascii-alpha-numeric</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ascii-alpha-numeric" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L105" title="View source at line 105">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-ascii-digit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ascii-digit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ascii-digit" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

The ASCII digit and hex-digit test/value pairs. Every text and
data home that parses numeric text had written its own (under
`is-digit-char`, `digit-of`, `is-hex`, `hex-of`, ...), and this is
their one home. The names say ASCII because that is the whole
class they accept: there is no Unicode digit or decimal-digit
database here, and none is claimed.

</div>
</article>

<article class="doc-entry" id="v-is-ascii-hex-digit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ascii-hex-digit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ascii-hex-digit" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L76" title="View source at line 76">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-ascii-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ascii-lower</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ascii-lower" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L97" title="View source at line 97">Source</a></span></div>
</article>

<article class="doc-entry" id="v-is-ascii-space">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ascii-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ascii-space" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

ASCII whitespace: space, tab, LF, VT, FF, CR (the ASCII subset of
base's `isSpace`; the other three controls are the whitespace
classes that exist outside ASCII).

</div>
</article>

<article class="doc-entry" id="v-is-ascii-upper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-ascii-upper</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-ascii-upper" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

A-Z is `65..90`, `a`-`z` is `97..122` in code point order.

</div>
</article>

<article class="doc-entry" id="v-le-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">le-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-le-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L30" title="View source at line 30">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lt-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lt-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-lt-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L29" title="View source at line 29">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ne-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ne-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-ne-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L28" title="View source at line 28">Source</a></span></div>
</article>

<article class="doc-entry" id="v-to-lower-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-lower-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-to-lower-ascii" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L128" title="View source at line 128">Source</a></span></div>
</article>

<article class="doc-entry" id="v-to-upper-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-upper-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-to-upper-ascii" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

The ASCII case fold. A scalar outside the affected range is
returned unchanged, which is the documented behaviour for every
non-ASCII scalar and every non-letter. `char-from-code` cannot
fail on a code an upper/lower test just admitted, so the `None`
arm returns the input unchanged (never a magic placeholder).

</div>
</article>

<article class="doc-entry" id="v-u32-to-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">u32-to-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-u32-to-char" title="Permalink">#</a><a href="/packages/stdlib/Data.Char/source/#L26" title="View source at line 26">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Char"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Char.Eq_Char</code></span></li>
<li id="i-Hash_Char"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Char.Hash_Char</code></span><div class="doc-child-prose">The scalar value is the digest (instances live with their type).</div></li>
<li id="i-Ord_Char"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Char.Ord_Char</code></span></li>
</ul>
