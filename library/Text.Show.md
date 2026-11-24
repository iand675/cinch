# Text.Show

Rendering values as String. The concrete formatting helpers are re-exported by Prelude; import this module for Show itself.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Text.Show</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Show/source/">View source</a><a href="/packages/search/?q=%2BText.Show">Search module</a><a href="/reference-sources/Text.Show.cinch" download="Text.Show.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Text.Show: rendering values as `String`.

Module scheme (design/base-library.md): `Text.*` is conversion to
and from String/Bytes. The `Show` class is the conversion family's
class; there was no earlier `Show` to move (nothing in `Relation`
or elsewhere carried one), so it lands here in its scheme home,
refined to the bootstrap's minimal shape: `show` renders, with no
`shows`/`showList` zoo.

The concrete conversions below (`nat-to-string`, `int-to-string`,
`char-string`) are the prelude surface's formatting family (their
names are decision D106's dash style and unchanged by the split).
Their digits are raw ASCII bytes (48..=57), so no character
literals are needed.

Instance homes (documented deviation from "instances live with
their type"): the `Show` instances for the primitive carriers live
HERE with the conversion family, not in their Data.* vocabulary
modules. `show`'s result type `String` sits above the bignum and
byte operations in the module order (Data.String's UTF-8 engine
needs the `Nat` operations), so `Show Nat` in Data.Nat would close
an import cycle. `Show` for `Bool`/`Ordering` renders their fixed
text through string literals, which ordinary terms now carry.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Show">Show</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Show</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Show-show">show</a><code class="syn-sig"><strong class="tok-variable">show</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-char-string">char-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">char-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-int-to-string">int-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-nat-digit-count">nat-digit-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-digit-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-digits-into">nat-digits-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-digits-into</strong> :: <span class="tok-variable">arg1</span> + <span class="tok-variable">arg3</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-nat-to-string">nat-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pow10">pow10</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pow10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Show">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Show</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Show" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Show-show"><div class="doc-method-row"><code><strong class="tok-variable">show</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#m-Show-show" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L36">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (9)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sym/#instances">Data.Sym</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Wide/#instances">Data.Wide</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Wide/#instances">Data.Wide</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-char-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">char-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-char-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L76" title="View source at line 76">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-int-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Signed decimal for `Int`, built from bignum division so values
beyond i64 format correctly: the magnitude is a `Nat` (`0 - x` for
a negative `x`) and renders as one, after a raw ASCII 45 byte for
the sign.

</div>
</article>

<article class="doc-entry" id="v-nat-digit-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-digit-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-digit-count" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L99" title="View source at line 99">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nat-digits-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-digits-into</strong> :: <span class="tok-variable">arg1</span> + <span class="tok-variable">arg3</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-nat-digits-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Decimal formatting: digits are raw ASCII bytes (48..=57), so no
character literals are needed. The low `k` digits of `n` fill
[pos, pos + k) from the END, one division per digit; extracting each
digit from the front as `n / 10^(k - 1)` would rebuild the power and
divide the whole number at every digit, quadratic in `k`. The span
lies inside the buffer, and each digit is `48 + n % 10 <= 57`.

</div>
</article>

<article class="doc-entry" id="v-nat-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-nat-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L102" title="View source at line 102">Source</a></span></div>
</article>

<article class="doc-entry" id="v-pow10">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pow10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pow10" title="Permalink">#</a><a href="/packages/stdlib/Text.Show/source/#L96" title="View source at line 96">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Show_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Show.Show_Bool</code></span><div class="doc-child-prose">The fixed text: each renders as its literal constructor spelling (probed in seed/tests/stringliterals.rs).</div></li>
<li id="i-Show_Char"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Show.Show_Char</code></span></li>
<li id="i-Show_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Show.Show_Int</code></span></li>
<li id="i-Show_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Show.Show_Nat</code></span><div class="doc-child-prose">The decimal rendering agrees with the concrete conversions below (the class's law shape: show is the rendering; asserted over sample values in seed/tests/preludesplit.rs).</div></li>
<li id="i-Show_Ordering"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Show.Show_Ordering</code></span></li>
<li id="i-Show_String"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Show.Show_String</code></span><div class="doc-child-prose">The raw text: String renders as itself (no quoting in the bootstrap).</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Show.cinch`](/packages/stdlib/Text.Show/source/). Source SHA-256: `482adc06340f899b8664bfbf7804b45d74db5e200f657faa12a8eb5b66de76e8`.
