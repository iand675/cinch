# Data.Complex

Binary64 complex arithmetic, overflow-safe division, polar conversions, and full elementary, trigonometric, hyperbolic, and inverse functions over Data.Float.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Complex</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Complex/source/">View source</a><a href="/packages/search/?q=%2BData.Complex">Search module</a><a href="/reference-sources/Data.Complex.cinch" download="Data.Complex.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Float](Data.Float.md), [Data.Machine](Data.Machine.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

Complex numbers over the binary64 scalar (`F64`) with full elementary,
trigonometric, hyperbolic, inverse, polar and classification operations.

The carrier is fixed to `F64` (`Data.Machine`'s binary64 scalar): both parts
are machine floats, so mixed real/complex arithmetic is refused (`from-real`,
`real-part` and `imag-part` are the crossings). The `MkComplex` constructor
stays module-private; `complex` is the smart constructor.

Special-value handling conforms to C99 Annex G (IEC 60559 complex arithmetic):
signed zeros are preserved, branch cuts match standard principal values,
division uses Smith's scaling algorithm with infinity/zero recovery, `abs`
delegates to `hypot-f64`, `sqrt` rescales to avoid intermediate overflow or
underflow, and `tan`/`cot` use Cody-Waite and Payne-Hanek range reduction
with Taylor series evaluation near poles.

Class instances (`Eq`, `Add`, `Sub`, `Mul`, `Neg`) live with the type; `Eq`
is componentwise IEEE equality via `feq-f64` and `Complex` deliberately has
no `Ord` or `Div` instance.

</details>

<details class="doc-synopsis"><summary>Synopsis (42 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Complex">Complex</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Complex</strong></code></li><li><a class="syn-name" href="#v-abs">abs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-acos">acos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">acos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-acosh">acosh</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">acosh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-add">add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-arg">arg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">arg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-asin">asin</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-asinh">asinh</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asinh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-atan">atan</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-atan-approx">atan-approx</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atan-approx</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-atanh">atanh</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atanh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-complex">complex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">complex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-complex-inf">complex-inf</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">complex-inf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-complex-nan">complex-nan</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">complex-nan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-conj">conj</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">conj</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-conjugate">conjugate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">conjugate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-cos">cos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-cosh">cosh</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cosh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-cot">cot</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-div">div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-eq">eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-exp">exp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">exp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-from-real">from-real</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-real</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-imag-part">imag-part</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">imag-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-is-inf">is-inf</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-inf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-nan">is-nan</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-nan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-log">log</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-log10">log10</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-mul">mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-neg">neg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-phase">phase</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">phase</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-polar">polar</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">polar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-pow">pow</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-real-part">real-part</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">real-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-rect">rect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-sin">sin</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-sinh">sinh</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sinh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-sqrt">sqrt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sqrt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-sub">sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-tan">tan</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-tanh">tanh</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tanh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-to-string">to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Complex">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Complex</strong></code><span class="doc-sig-links"><a href="#t-Complex" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

A complex number with binary64 real and imaginary parts.

<details class="doc-instances"><summary>Instances (5)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-abs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-abs" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

The modulus `|a + bi|`, computed overflow- and underflow-safely via `hypot-f64`.

</div>
</article>

<article class="doc-entry" id="v-acos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">acos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-acos" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L627" title="View source at line 627">Source</a></span></div>
<div class="doc-body">

Complex inverse circular cosine.

</div>
</article>

<article class="doc-entry" id="v-acosh">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">acosh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-acosh" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L633" title="View source at line 633">Source</a></span></div>
<div class="doc-body">

Complex inverse hyperbolic cosine.

</div>
</article>

<article class="doc-entry" id="v-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Componentwise addition `(a + bi) + (c + di)`.

</div>
</article>

<article class="doc-entry" id="v-arg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">arg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-arg" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L215" title="View source at line 215">Source</a></span></div>
<div class="doc-body">

The four-quadrant argument of a complex number in `[-pi, pi]`.

</div>
</article>

<article class="doc-entry" id="v-asin">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-asin" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L576" title="View source at line 576">Source</a></span></div>
<div class="doc-body">

Complex inverse circular sine.

</div>
</article>

<article class="doc-entry" id="v-asinh">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asinh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-asinh" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L603" title="View source at line 603">Source</a></span></div>
<div class="doc-body">

Complex inverse hyperbolic sine.

</div>
</article>

<article class="doc-entry" id="v-atan">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-atan" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L645" title="View source at line 645">Source</a></span></div>
<div class="doc-body">

Complex inverse circular tangent.

</div>
</article>

<article class="doc-entry" id="v-atan-approx">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atan-approx</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-atan-approx" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L680" title="View source at line 680">Source</a></span></div>
<div class="doc-body">

Named arctangent approximation over `(-pi/2, pi/2)` retained for compatibility.

</div>
</article>

<article class="doc-entry" id="v-atanh">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atanh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-atanh" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L674" title="View source at line 674">Source</a></span></div>
<div class="doc-body">

Complex inverse hyperbolic tangent.

</div>
</article>

<article class="doc-entry" id="v-complex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">complex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-complex" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Construct a complex number from its real and imaginary parts.

</div>
</article>

<article class="doc-entry" id="v-complex-inf">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">complex-inf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-complex-inf" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Complex infinity `(+Inf, +Inf)`.

</div>
</article>

<article class="doc-entry" id="v-complex-nan">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">complex-nan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-complex-nan" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Complex not-a-number `(NaN, NaN)`.

</div>
</article>

<article class="doc-entry" id="v-conj">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">conj</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-conj" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L199" title="View source at line 199">Source</a></span></div>
<div class="doc-body">

The complex conjugate `a - bi` (alias of `conjugate`).

</div>
</article>

<article class="doc-entry" id="v-conjugate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">conjugate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-conjugate" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

The complex conjugate `a - bi` (reflection across the real axis).

</div>
</article>

<article class="doc-entry" id="v-cos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-cos" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L392" title="View source at line 392">Source</a></span></div>
<div class="doc-body">

Complex circular cosine.

</div>
</article>

<article class="doc-entry" id="v-cosh">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cosh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-cosh" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L416" title="View source at line 416">Source</a></span></div>
<div class="doc-body">

Complex hyperbolic cosine.

</div>
</article>

<article class="doc-entry" id="v-cot">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-cot" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L566" title="View source at line 566">Source</a></span></div>
<div class="doc-body">

Complex circular cotangent.

</div>
</article>

<article class="doc-entry" id="v-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-div" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

Overflow-safe complex division using Smith's scaling algorithm with C99
Annex G.5.1 infinity and zero recovery.

</div>
</article>

<article class="doc-entry" id="v-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-eq" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Structural equality: componentwise IEEE equality via `feq-f64`.

</div>
</article>

<article class="doc-entry" id="v-exp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">exp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-exp" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L235" title="View source at line 235">Source</a></span></div>
<div class="doc-body">

Complex base-e exponential `e^z`.

</div>
</article>

<article class="doc-entry" id="v-from-real">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-real</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-from-real" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

Lift a real `F64` into `Complex` with zero imaginary part.

</div>
</article>

<article class="doc-entry" id="v-imag-part">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">imag-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-imag-part" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

The imaginary part of a complex number.

</div>
</article>

<article class="doc-entry" id="v-is-inf">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-inf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-inf" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Report whether either part is an infinity.

</div>
</article>

<article class="doc-entry" id="v-is-nan">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-nan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-nan" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Report whether either part is NaN and neither part is an infinity.

</div>
</article>

<article class="doc-entry" id="v-log">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-log" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L251" title="View source at line 251">Source</a></span></div>
<div class="doc-body">

Principal natural logarithm `log(|z|) + i * phase(z)`.

</div>
</article>

<article class="doc-entry" id="v-log10">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-log10" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

Principal base-10 logarithm `log(z) / ln(10)`.

</div>
</article>

<article class="doc-entry" id="v-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">

Complex multiplication `(a + bi)(c + di) = (ac - bd) + (ad + bc)i`.

</div>
</article>

<article class="doc-entry" id="v-neg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-neg" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Componentwise additive inverse `-(a + bi) = -a - bi`.

</div>
</article>

<article class="doc-entry" id="v-phase">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">phase</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-phase" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L211" title="View source at line 211">Source</a></span></div>
<div class="doc-body">

The phase (argument) of a complex number in `[-pi, pi]`, computed via `atan2-f64`.

</div>
</article>

<article class="doc-entry" id="v-polar">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">polar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-polar" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

Polar representation `(r, theta)` such that `z = r * e^(i * theta)` with `theta` in `[-pi, pi]`.

</div>
</article>

<article class="doc-entry" id="v-pow">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-pow" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

Complex exponentiation `x^y`.

</div>
</article>

<article class="doc-entry" id="v-real-part">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">real-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-real-part" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

The real part of a complex number.

</div>
</article>

<article class="doc-entry" id="v-rect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-rect" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L223" title="View source at line 223">Source</a></span></div>
<div class="doc-body">

Construct a complex number from polar coordinates `(r, theta)`.

</div>
</article>

<article class="doc-entry" id="v-sin">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-sin" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L344" title="View source at line 344">Source</a></span></div>
<div class="doc-body">

Complex circular sine.

</div>
</article>

<article class="doc-entry" id="v-sinh">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sinh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-sinh" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L368" title="View source at line 368">Source</a></span></div>
<div class="doc-body">

Complex hyperbolic sine.

</div>
</article>

<article class="doc-entry" id="v-sqrt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sqrt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-sqrt" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L288" title="View source at line 288">Source</a></span></div>
<div class="doc-body">

Principal square root of `z`, with `real-part r >= 0` and `imag-part r` matching the sign of `imag-part z`.

</div>
</article>

<article class="doc-entry" id="v-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Componentwise subtraction `(a + bi) - (c + di)`.

</div>
</article>

<article class="doc-entry" id="v-tan">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-tan" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L531" title="View source at line 531">Source</a></span></div>
<div class="doc-body">

Complex circular tangent.

</div>
</article>

<article class="doc-entry" id="v-tanh">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tanh</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-tanh" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L549" title="View source at line 549">Source</a></span></div>
<div class="doc-body">

Complex hyperbolic tangent.

</div>
</article>

<article class="doc-entry" id="v-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L738" title="View source at line 738">Source</a></span></div>
<div class="doc-body">

Format a complex number in `a+bi` or `a-bi` form.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Add_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Add_Complex</code></span><div class="doc-child-prose">Componentwise addition for Complex.</div></li>
<li id="i-Eq_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Eq_Complex</code></span><div class="doc-child-prose">Componentwise IEEE equality for Complex.</div></li>
<li id="i-Mul_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Mul_Complex</code></span><div class="doc-child-prose">Complex multiplication for Complex.</div></li>
<li id="i-Neg_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Neg_Complex</code></span><div class="doc-child-prose">Componentwise negation for Complex.</div></li>
<li id="i-Sub_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Sub_Complex</code></span><div class="doc-child-prose">Componentwise subtraction for Complex.</div></li>
</ul>
