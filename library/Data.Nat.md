# Data.Nat

Mathematical nonnegative integers, exact arithmetic, checked preconditions, and admitted machine conversions.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Nat</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Nat/source/">View source</a><a href="/packages/search/?q=%2BData.Nat">Search module</a><a href="/reference-sources/Data.Nat.cinch" download="Data.Nat.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Semiring](Algebra.Semiring.md), [Data.Core](Data.Core.md), [Data.Enum](Data.Enum.md), [Data.Hash](Data.Hash.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Nat: the mathematical natural numbers.

`Nat` is a core runtime type (primitive types specification section
1): a boxed bignum, distinct from every machine scalar. The runtime
operations are representation-level and declared through `foreign
import`. Division and remainder are total by construction: the
divisor is a `NonZero Nat` (a numeral divisor decides at compile
time, any other comes from `decide (NonZero d)` or is threaded), and
`checked-div`/`checked-rem` answer `None` where a zero divisor is a
meaningful input. Subtraction is total the same way
(design/no-faults.md section 7): `a - b` and `nat-sub a b` require
evidence that `b <= a` (numerals decide it at compile time; a branch
condition such as `if b <= a then` supplies it), `saturating-sub`
is the truncating form (`max(a - b, 0)`, "monus") under its own
name, and `checked-sub` answers `None` when `b > a`.

Module scheme (design/base-library.md): the type's home carries its
instance family. `Nat` gets no `Semigroup` instance: two monoids
live on it and the newtypes `Sum`/`Product` choose (Algebra.Monoid).
`Semiring` names BOTH operations, so the carrier itself can be a
semiring: this is the `Sum`/`Product` story extended (no Num tower;
machine scalars keep their explicit overflow families and get no
`Semiring` instance).

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-checked-div">checked-div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">checked-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-checked-nat-from-i64">checked-nat-from-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">checked-nat-from-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-checked-nat-to-i64">checked-nat-to-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">checked-nat-to-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-checked-rem">checked-rem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">checked-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-checked-sub">checked-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-add">nat-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-cmp">nat-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-nat-div">nat-div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-from-i64">nat-from-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-from-i64</strong> :: 0 &lt;= <span class="tok-variable">arg0</span> <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-from-int">nat-from-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-from-int</strong> :: 0 &lt;= <span class="tok-variable">arg0</span> <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-mul">nat-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-rem">nat-rem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">cinch-result</span> &lt; <span class="tok-variable">d</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-sub">nat-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-sub</strong> :: <span class="tok-variable">arg1</span> &lt;= <span class="tok-variable">arg0</span> <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nat-to-i64">nat-to-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-to-i64</strong> :: <span class="tok-variable">arg0</span> &lt;= 9223372036854775807 <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-nat-to-int">nat-to-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nat-to-int</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">cinch-result</span> == <span class="tok-variable">n</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-saturating-sub">saturating-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-checked-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">checked-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-checked-div" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L64" title="View source at line 64">Source</a></span></div>
<div class="doc-body">

Division where a zero divisor is an ordinary input the caller
handles: `None` exactly when `d` is zero. The decision is the
divisor's evidence, so the division inside is the total one.

</div>
</article>

<article class="doc-entry" id="v-checked-nat-from-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">checked-nat-from-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-checked-nat-from-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

The `I64` conversions where an out-of-range value is an ordinary
input: `None` exactly then.

</div>
</article>

<article class="doc-entry" id="v-checked-nat-to-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">checked-nat-to-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-checked-nat-to-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L90" title="View source at line 90">Source</a></span></div>
</article>

<article class="doc-entry" id="v-checked-rem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">checked-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-checked-rem" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L69" title="View source at line 69">Source</a></span></div>
</article>

<article class="doc-entry" id="v-checked-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-checked-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Subtraction where `b > a` is an ordinary input the caller handles:
`None` exactly then. The test is the subtraction's evidence.

</div>
</article>

<article class="doc-entry" id="v-nat-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

The mathematical `Nat` and `Int` (primitive types specification,
section 1): boxed bignums, distinct from every machine scalar.

</div>
</article>

<article class="doc-entry" id="v-nat-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-nat-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L46" title="View source at line 46">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nat-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-div" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L42" title="View source at line 42">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nat-from-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-from-i64</strong> :: 0 &lt;= <span class="tok-variable">arg0</span> <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-from-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

The `I64` conversions keep the value (`n == x`), and each states
the range it needs: a negative `I64` is no `Nat`, and a `Nat` past
`2^63 - 1` is no `I64`. A conversion result (`u32-to-i64`), a mask
(`mask-i64 x 255`) or a guard supplies the evidence; where the
value may be out of range, `checked-nat-from-i64`/
`checked-nat-to-i64` answer `None`.

</div>
</article>

<article class="doc-entry" id="v-nat-from-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-from-int</strong> :: 0 &lt;= <span class="tok-variable">arg0</span> <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-from-int" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L59" title="View source at line 59">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nat-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L41" title="View source at line 41">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nat-rem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">cinch-result</span> &lt; <span class="tok-variable">d</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-rem" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

The remainder is below the divisor (`r < d`, a fact about each
call's result: `let r = nat-rem v 256` knows `r < 256`).

</div>
</article>

<article class="doc-entry" id="v-nat-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-sub</strong> :: <span class="tok-variable">arg1</span> &lt;= <span class="tok-variable">arg0</span> <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nat-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

The exact difference: the caller proves it is a `Nat`.

</div>
</article>

<article class="doc-entry" id="v-nat-to-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-to-i64</strong> :: <span class="tok-variable">arg0</span> &lt;= 9223372036854775807 <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> == <span class="tok-variable">arg0</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-nat-to-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L54" title="View source at line 54">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nat-to-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nat-to-int</strong> :: (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">cinch-result</span> == <span class="tok-variable">n</span>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-nat-to-int" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

The `Int` conversions: every `Nat` is an `Int`, and a non-negative
`Int` is a `Nat`, both keeping the value (no `I64` in between, so a
value of any size converts).

</div>
</article>

<article class="doc-entry" id="v-saturating-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-saturating-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Nat/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Truncated subtraction ("monus", Rust's `saturating_sub`): `a - b`
when `b <= a`, else `0`. Use it where clamping at zero is what the
code means, not to silence the obligation of `-`.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Add_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Add_Nat</code></span></li>
<li id="i-Div_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Div_Nat</code></span></li>
<li id="i-Enum_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Enum_Nat</code></span><div class="doc-child-prose">The enumeration of natural numbers (Data.Enum): codes are the values; pred 0 and to-enum of a negative code are None (instances live with their type).</div></li>
<li id="i-Eq_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Eq_Nat</code></span></li>
<li id="i-Hash_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Hash_Nat</code></span><div class="doc-child-prose">A Nat is its own bucket digest (identity hashing; instances live with their type).</div></li>
<li id="i-Mul_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Mul_Nat</code></span></li>
<li id="i-Ord_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Ord_Nat</code></span></li>
<li id="i-Rem_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Rem_Nat</code></span></li>
<li id="i-Semiring_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Semiring_Nat</code></span><div class="doc-child-prose">The semiring of natural numbers: addition and multiplication with their identities (see the header; instances live with their type).</div></li>
<li id="i-Sub_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Nat.Sub_Nat</code></span><div class="doc-child-prose">- at Nat is the exact difference, so this instance states the precondition over the method's arguments: every direct x - y discharges y &lt;= x (a numeral pair decides it, a branch fact or Dec evidence supplies it).</div></li>
</ul>
