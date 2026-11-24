# Data.Complex

Complex values and numeric operations using the module's current floating representation.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Complex</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Complex/source/">View source</a><a href="/packages/search/?q=%2BData.Complex">Search module</a><a href="/reference-sources/Data.Complex.cinch" download="Data.Complex.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Complex: complex numbers over the binary64 scalar.

Ruby's `Complex` (design/stdlib-parity-ruby.md, the P38 numeric family:
PORT as a concrete type with explicit operations, the P2 discipline of
types and operations rather than a numeric tower). The carrier is fixed
to `F64` (Data.Machine's binary64 scalar): both parts are machine floats,
so the operations below are the field operations named explicitly rather
than a promotion system. `complex` is the smart constructor (the
representation stays private, like `Data.String`'s `StrBuf`), and
`real-part`/`imag-part` are the observations that cross back to `F64`.

Design law shape (design/base-library.md: `Data.*` is concrete types, and
the type's home carries its instance family): the defining identities the
tests assert are
  the imaginary unit:   mul (complex 0 1) (complex 0 1) == complex -1 0
  the sample product:   mul (complex 1 2) (complex 3 4) == complex -5 10
  distributivity:       mul x (add y z) == add (mul x y) (mul x z)
  conjugation:          conjugate (conjugate z) == z
  the field inverse:    div (mul x y) y == x within the tolerance, where
                        y is not zero (division by zero is IEEE, not a
                        fault: the parts go through `div-f64`)
and the instance family (`Eq`, `Add`, `Sub`, `Mul`, `Neg`) is the
class surface over those named functions. The arithmetic-operation
classes DO admit this carrier: the class method is over `Complex`, not
over `F64`, so `Add Complex` adds both parts with `add-f64` (the refusal
of arithmetic instances on the machine scalars themselves, primitive
types specification section 8, is not touched).

Named refusals:
  * No `Complex` over `Int` (nor any carrier but `F64`): the type is
    fixed here. Ruby's `Complex` promotes across `Numeric`; this slice
    deliberately does not, and mixed real/complex arithmetic is refused
    too (`from-real`, `real-part` and `imag-part` are the crossings).
  * No ordering: comparison is absent on purpose. The parts are `F64`,
    which has no `Ord` instance (primitive types specification section
    8), and there is no total order on the complex field compatible with
    its arithmetic. The bit-pattern order `total-order-f64` orders NaN
    and signed zeros, not values, so it is not an ordering either.
  * `Eq` is structural on the parts through `feq-f64`, so a value with a
    NaN part is unequal to itself (IEEE equality, matching Ruby's
    componentwise `==`).
  * No `Div` instance: `Algebra.Arithmetic`'s `(/)` divides by a
    `NonZero` divisor (division is total by construction), and the
    predicate `NonZero` decides numbers, not `Complex`. Complex
    division is total in its own way (IEEE parts, never a fault), so
    it stays the named function `div`.
  * No `Semiring`/`Group` instances: this module provides exactly the
    arithmetic-operation classes (`Algebra.Arithmetic`) and leaves the
    algebraic-structure classes to a later curation (a class constraint
    entails nothing, so nothing is lost).
  * `to-string` renders each part through the P31 round-trip decimal
    (Data.Machine's `f64-to-decimal`: the shortest round-trip spelling)
    with a trailing `e0` elided, so whole and simple fractional values
    read as plain decimals. A part whose decimal needs a nonzero
    exponent keeps its scientific spelling; no fixed-notation expansion
    is attempted in this slice.
  * `arg`/`atan-approx` are a named APPROXIMATION. The binary64 scalar
    carries no arctangent primitive (Data.Machine has no `atan`), so
    `atan-approx` reduces the argument four times through
    `t/(1+sqrt(1+t^2))` (each reduction halves the angle) and sums the
    first eight odd terms of the arctangent series. The absolute error
    is below 1e-12 over the whole real line: the reduced truncation (the
    reduced argument is at most 2^-4) is below 2^-60 and the four
    doublings scale it by 2^4, with the `pi/2` constant adding binary64
    rounding noise. `arg` is the four-quadrant `atan2` built on it.

Instance homes. Instances live with their type (design/base-library.md);
the class names come through the implicit prelude facade, the same shape
Data.Int's instances use. Names are dash style (D106); operators are
class methods with the base fixity table (D2).

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Complex">Complex</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Complex</strong></code></li><li><a class="syn-name" href="#v-abs">abs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-add">add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-arg">arg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">arg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-atan-approx">atan-approx</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atan-approx</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-complex">complex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">complex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-conjugate">conjugate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">conjugate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-div">div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-eq">eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-from-real">from-real</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-real</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-imag-part">imag-part</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">imag-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-mul">mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-neg">neg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-real-part">real-part</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">real-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-sub">sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code></li><li><a class="syn-name" href="#v-to-string">to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Complex">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Complex</strong></code><span class="doc-sig-links"><a href="#t-Complex" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

The pair of binary64 parts. The constructor is module-private (the
observations below are the only way out); `complex` is the surface.

<details class="doc-instances"><summary>Instances (5)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-abs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-abs" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

The magnitude |a + bi| = sqrt(a^2 + b^2). Naive formula (no scaling):
a part large enough to overflow the squares yields an infinite
magnitude, which is the honest IEEE result for this spelling.

</div>
</article>

<article class="doc-entry" id="v-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

(a + bi) + (c + di): componentwise (the field's addition).

</div>
</article>

<article class="doc-entry" id="v-arg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">arg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-arg" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L228" title="View source at line 228">Source</a></span></div>
<div class="doc-body">

The four-quadrant argument: the angle of `a + bi` in (-pi, pi], zero
for the origin. The axis cases are exact; the other quadrants defer to
`atan-approx` (the approximation's bound is the error bound here).

</div>
</article>

<article class="doc-entry" id="v-atan-approx">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atan-approx</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-atan-approx" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

The named arctangent approximation (the header's refusal entry):
`atan-approx x` is the principal value in (-pi/2, pi/2).

</div>
</article>

<article class="doc-entry" id="v-complex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">complex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-complex" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

The smart constructor (Ruby's `Complex(a, b)`).

</div>
</article>

<article class="doc-entry" id="v-conjugate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">conjugate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-conjugate" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

The conjugate: the reflection across the real axis. An involution
(`conjugate (conjugate z) == z`).

</div>
</article>

<article class="doc-entry" id="v-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-div" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

(a + bi)/(c + di) = ((ac + bd) + (bc - ad)i) / (c^2 + d^2). A zero
divisor yields IEEE NaN parts through `div-f64`, never a fault.

</div>
</article>

<article class="doc-entry" id="v-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-eq" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Structural equality: IEEE on each part.

</div>
</article>

<article class="doc-entry" id="v-from-real">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-real</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-from-real" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

A complex number with zero imaginary part.

</div>
</article>

<article class="doc-entry" id="v-imag-part">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">imag-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-imag-part" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L94" title="View source at line 94">Source</a></span></div>
</article>

<article class="doc-entry" id="v-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

(a + bi)(c + di) = (ac - bd) + (ad + bc)i (the imaginary unit squares
to -1, which is the `mul (complex 0 1) (complex 0 1) == complex -1 0`
identity above).

</div>
</article>

<article class="doc-entry" id="v-neg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-neg" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

The additive inverse, componentwise.

</div>
</article>

<article class="doc-entry" id="v-real-part">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">real-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-real-part" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L90" title="View source at line 90">Source</a></span></div>
</article>

<article class="doc-entry" id="v-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code><span class="doc-sig-links"><a href="#v-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

(a + bi) - (c + di): componentwise.

</div>
</article>

<article class="doc-entry" id="v-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Complex/source/#L257" title="View source at line 257">Source</a></span></div>
<div class="doc-body">

The `a+bi` form: a negative imaginary part renders with `-` and its
magnitude (so `complex 1 -1` is `1-1i`), a non-negative one with `+`.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Add_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Add_Complex</code></span></li>
<li id="i-Eq_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Eq_Complex</code></span></li>
<li id="i-Mul_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Mul_Complex</code></span></li>
<li id="i-Neg_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Neg_Complex</code></span></li>
<li id="i-Sub_Complex"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Complex.Sub_Complex</code></span></li>
</ul>
