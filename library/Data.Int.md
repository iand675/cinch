# Data.Int

Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Int</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Int/source/">View source</a><a href="/packages/search/?q=%2BData.Int">Search module</a><a href="/reference-sources/Data.Int.cinch" download="Data.Int.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Group](Algebra.Group.md), [Algebra.Semiring](Algebra.Semiring.md), [Data.Core](Data.Core.md), [Data.Enum](Data.Enum.md), [Data.Hash](Data.Hash.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Int: the mathematical integers.

`Int` is a core runtime type (primitive types specification section
1): a boxed bignum, distinct from every machine scalar. Division
and remainder are total by construction: the divisor is a `NonZero
Int` (see Data.Nat).

Module scheme (design/base-library.md): the type's home carries its
instance family. `Int` gets no `Semigroup` instance (two monoids
live on it, chosen by `Sum`/`Product` in Algebra.Monoid); its
`Semiring` names both operations, and its `Group` is the additive
group of that semiring (`invert` is `int-neg`, the only total
inversion on `Int`).

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-checked-int-to-i64">checked-int-to-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">checked-int-to-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-int-add">int-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-int-cmp">int-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-int-div">int-div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-int-from-i64">int-from-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-from-i64</strong> :: (<span class="tok-variable">x</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">cinch-result</span> == <span class="tok-variable">x</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-int-mod-nat">int-mod-nat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-mod-nat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-int-mul">int-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-int-neg">int-neg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-int-rem">int-rem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-int-sub">int-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-int-to-i64">int-to-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-to-i64</strong> :: (- 9223372036854775808 &lt;= <span class="tok-variable">arg0</span>, <span class="tok-variable">arg0</span> &lt;= 9223372036854775807) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-checked-int-to-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">checked-int-to-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-checked-int-to-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L39" title="View source at line 39">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-int-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L25" title="View source at line 25">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-int-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L31" title="View source at line 31">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-int-div" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L28" title="View source at line 28">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-from-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-from-i64</strong> :: (<span class="tok-variable">x</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">cinch-result</span> == <span class="tok-variable">x</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-int-from-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

The `I64` conversions keep the value; an `Int` past the `I64`
range is no `I64`, so `int-to-i64` states the range it needs (a
guard or a conversion result supplies it) and `checked-int-to-i64`
answers `None` outside it.

</div>
</article>

<article class="doc-entry" id="v-int-mod-nat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-mod-nat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-int-mod-nat" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

The floored residue of `x` modulo `d` as a `Nat`, `0 <= r < d`
for every `x` (a negative `x` counts up from `d`): an encoding into
`d` values that needs no range of `x`.

</div>
</article>

<article class="doc-entry" id="v-int-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-int-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L27" title="View source at line 27">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-neg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-int-neg" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L30" title="View source at line 30">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-rem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-int-rem" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L29" title="View source at line 29">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-int-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L26" title="View source at line 26">Source</a></span></div>
</article>

<article class="doc-entry" id="v-int-to-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-to-i64</strong> :: (- 9223372036854775808 &lt;= <span class="tok-variable">arg0</span>, <span class="tok-variable">arg0</span> &lt;= 9223372036854775807) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-int-to-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Int/source/#L37" title="View source at line 37">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Add_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Add_Int</code></span></li>
<li id="i-Div_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Div_Int</code></span></li>
<li id="i-Enum_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Enum_Int</code></span><div class="doc-child-prose">The enumeration of mathematical integers (Data.Enum): total in both directions, codes consecutive (instances live with their type).</div></li>
<li id="i-Eq_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Eq_Int</code></span><div class="doc-child-prose">Mathematical Int and Nat instances (primitive types specification, section 1): comparison through the cmp prims' immediates, arithmetic through the runtime.</div></li>
<li id="i-Group_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Group_Int</code></span><div class="doc-child-prose">The additive group of the integers: invert x is -x (the group inverse laws are asserted over samples in seed/tests/preludesplit.rs; instances live with their type).</div></li>
<li id="i-Hash_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Hash_Int</code></span><div class="doc-child-prose">The magnitude's digest: equal integers hash alike (the Hash law holds by construction; instances live with their type).</div></li>
<li id="i-Mul_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Mul_Int</code></span></li>
<li id="i-Neg_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Neg_Int</code></span></li>
<li id="i-Ord_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Ord_Int</code></span></li>
<li id="i-Rem_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Rem_Int</code></span></li>
<li id="i-Semiring_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Semiring_Int</code></span><div class="doc-child-prose">The semiring of integers: addition and multiplication with their identities (see the header).</div></li>
<li id="i-Sub_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Int.Sub_Int</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Int.cinch`](/packages/stdlib/Data.Int/source/). Source SHA-256: `4936154410419ff4f5a7b052e2e06ad5d272f541cfd026867f4192549b8a3319`.
