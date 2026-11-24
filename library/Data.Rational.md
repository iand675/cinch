# Data.Rational

Rational numbers with explicit denominator/error policy and arithmetic operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Rational</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Rational/source/">View source</a><a href="/packages/search/?q=%2BData.Rational">Search module</a><a href="/reference-sources/Data.Rational.cinch" download="Data.Rational.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Hash](Data.Hash.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Rational: the exact rational number over the bignum `Int`.

A `Rational` is an integer pair `n/d` kept NORMALISED: the
denominator is strictly positive and the two are coprime, so equal
mathematical values have exactly one representative and `Eq` is
structural. This is Ruby's `Rational` (design/stdlib-parity-ruby.md,
P38 PORT, "Rational ... as concrete types with explicit operations,
the P2 discipline: types and operations, no numeric tower"):
`Rational(1,2) == Rational(2,4)`, the sign rides on the numerator
(`Rational(-1,2).denominator` is `2`), and arithmetic is exact
because it is bignum, never a machine float.

Design law shape. The operations are named functions over one
concrete type (no `Fractional` class, no numeric tower): `add`,
`sub` and `mul` are total and exact, `from-int` embeds the integers,
and the order is cross multiplication (`rational-cmp r s` has the
sign of `n r * d s` against `n s * d r`, both denominators positive).
The `Eq` and `Hash` instances are consistent with reduction by
construction: the only route to a value normalises it, so `r == s`
exactly when their unique representatives agree, and equal values
hash alike.

The type is ABSTRACT. The raw constructor stays off the export list
(the surface rule D143 and the constructor privacy of D308), so a
`Rational` violating the invariant is unrepresentable: `rational`
is the only general constructor, `from-int` cannot fail, and
`numerator`/`denominator` are the only observers of the pair.

Named refusals (each a named value, never a silent fault; Ruby's
`ZeroDivisionError` in the two places valuation reaches it):
  a zero denominator cannot be constructed: `rational n 0` is
    `Err ZeroDenominator`;
  a zero divisor cannot be inverted: `div x y` with `y` equal to
    `from-int 0` is `Err DivideByZero`. `div` is the only partial
    operation; `add`/`sub`/`mul` are total.

`to-f64` boundary (documented, not silent): a rational whose
numerator and denominator each fit `I64` divides their two nearest
`F64` values, so the result is within two units in the last place of
the exact quotient, and exact when both magnitudes are at most 2^53.
Outside `I64` each is assembled from 32-bit limbs (`int-to-f64`), so
a huge one is a rounded float or an infinity rather than a fault.

Module scheme (design/base-library.md): `Data.*` is concrete types,
and the type's home carries its `Eq`/`Ord`/`Hash` instance family.

</details>

<details class="doc-synopsis" open><summary>Synopsis (14 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Rational">Rational</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rational</strong></code></li><li><a class="syn-name" href="#t-RationalError">RationalError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RationalError</strong></code></li><li><a class="syn-name" href="#v-add">add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code></li><li><a class="syn-name" href="#v-denominator">denominator</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">denominator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-div">div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code></li><li><a class="syn-name" href="#v-from-int">from-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code></li><li><a class="syn-name" href="#v-mul">mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code></li><li><a class="syn-name" href="#v-numerator">numerator</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">numerator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-rational">rational</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rational</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code></li><li><a class="syn-name" href="#v-rational-cmp">rational-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rational-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-render-rational-error">render-rational-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-rational-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sub">sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code></li><li><a class="syn-name" href="#v-to-f64">to-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-to-string">to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Rational">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rational</strong></code><span class="doc-sig-links"><a href="#t-Rational" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

The rational number: `Rat n d` is `n/d`, `d` strictly positive and
`gcd(|n|, d)` equal to 1. The constructor is module-private: only
`normalise` (reached through `rational`, `from-int` and the
operations) builds one, so every value the surface hands out
satisfies the invariant.

<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Rational/#instances">Data.Rational</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Rational/#instances">Data.Rational</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Rational/#instances">Data.Rational</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-RationalError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RationalError</strong></code><span class="doc-sig-links"><a href="#t-RationalError" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Why a rational is not a value or is not invertible: both refusals
reach a zero denominator, and both are reported rather than
faulted.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZeroDenominator"><code><strong class="tok-constructor">ZeroDenominator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a></code></li><li id="k-DivideByZero"><code><strong class="tok-constructor">DivideByZero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code><span class="doc-sig-links"><a href="#v-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Exact arithmetic: one cross-product over the bignum, then one
reduction. The denominators are positive products, and `normalise`
resolves the sign the addition itself cannot introduce.

</div>
</article>

<article class="doc-entry" id="v-denominator">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">denominator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-denominator" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L129" title="View source at line 129">Source</a></span></div>
</article>

<article class="doc-entry" id="v-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code><span class="doc-sig-links"><a href="#v-div" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

Division, the one partial operation: a zero divisor (a zero
numerator on the right) is the named refusal, otherwise the cross
product `n1*d2 / d1*n2` normalises.

</div>
</article>

<article class="doc-entry" id="v-from-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code><span class="doc-sig-links"><a href="#v-from-int" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

The integer embedding `x/1`, already normalised (1 is coprime to
every integer).

</div>
</article>

<article class="doc-entry" id="v-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code><span class="doc-sig-links"><a href="#v-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L153" title="View source at line 153">Source</a></span></div>
</article>

<article class="doc-entry" id="v-numerator">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">numerator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-numerator" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L125" title="View source at line 125">Source</a></span></div>
</article>

<article class="doc-entry" id="v-rational">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rational</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code><span class="doc-sig-links"><a href="#v-rational" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

The smart constructor and the only general route to a `Rational`:
a zero denominator is the named refusal, everything else
normalises.

</div>
</article>

<article class="doc-entry" id="v-rational-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rational-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-rational-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Order by cross multiplication: both denominators are positive, so
`n1*d2` against `n2*d1` has the sign of `x - y`. `Relation.Ord`
carries no `cmp` method, so this named comparison is the spelling,
and the instance below derives the operators from it.

</div>
</article>

<article class="doc-entry" id="v-render-rational-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-rational-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-RationalError" title="Data.Rational.RationalError">RationalError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-rational-error" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L204" title="View source at line 204">Source</a></span></div>
</article>

<article class="doc-entry" id="v-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code><span class="doc-sig-links"><a href="#v-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L147" title="View source at line 147">Source</a></span></div>
</article>

<article class="doc-entry" id="v-to-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-to-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

The value as a machine float (see the boundary note in the
header).

</div>
</article>

<article class="doc-entry" id="v-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Rational/source/#L200" title="View source at line 200">Source</a></span></div>
<div class="doc-body">

The reduced rendering `n/d`: `1/2` for `Rational(1,2)`, `-1/2` for
`Rational(-1,2)`, and `n/1` (not `n`) for a whole number, Ruby's
spelling.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Rational"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Rational.Eq_Rational</code></span><div class="doc-child-prose">Structural equality IS value equality: the invariant makes the representative unique, so equal values compare equal.</div></li>
<li id="i-Hash_Rational"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Rational.Hash_Rational</code></span><div class="doc-child-prose">Equal values hash alike: the normalised pair is the canonical form, so mixing the two component hashes can never separate r from an r == s partner.</div></li>
<li id="i-Ord_Rational"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Rational.Ord_Rational</code></span><div class="doc-child-prose">The order of the line, through the named comparison above.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Rational.cinch`](/packages/stdlib/Data.Rational/source/). Source SHA-256: `87c8c6ce39162e1571a7f18ea2e6972fc514739a0d33788fb0ae0dfd557164ac`.
