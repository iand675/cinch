# Data.Float

Binary64 mathematics: roundings, roots, exponentials, logarithms, powers, trigonometric, hyperbolic and special functions (erf, Gamma, Bessel) and FMA as branch-free kernels, bit-identical on every target.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Float</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Float/source/">View source</a><a href="/packages/search/?q=%2BData.Float">Search module</a><a href="/reference-sources/Data.Float.cinch" download="Data.Float.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Data.Float`: [Data.Float](Data.Float.md).

```sh
cp seed/prelude/Data/Float.cinch ./Data.Float.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Binary64 mathematics: constants, classification, exact rounding and bit-level splitting, and the
elementary functions (roots, exponentials, logarithms, powers, trigonometric, inverse trigonometric and
hyperbolic functions).

Results are deterministic to the bit on every target: the functions use only IEEE-754 round-to-nearest
additions, multiplications, divisions and square roots (never fused multiply-adds or the host's libm), so the
same input gives the same output on 64-bit ARM, WebAssembly and big-endian machines alike. The algorithms
are the classic ones: argument reduction by a split ln 2 with minimax polynomials for exp, expm1, log and
log1p; rational approximations after reduction by a three-part pi/4 for the trigonometric functions, with
Payne-Hanek reduction against a 1216-bit 4/pi from 2^29 up; rational approximations for the arctangent and
the hyperbolic functions; and exact bit manipulation for frexp, ldexp, modf, nextafter and the roundings.
The special functions follow the same lineage: rational approximations for erf and erfc over five
intervals, the inverse error function by three rational fits, Gamma by its rational form on [2, 3) with
recurrence, Stirling's series and reflection, lgamma by interval polynomials and the reflection through
sin(pi x), Bessel functions of orders 0 and 1 by polynomials near zero and Hankel asymptotic forms beyond 2,
and of order n by forward recurrence, a continued fraction with backward recurrence, or the large-argument
form (J1 stays odd for tiny arguments: J1(x) = x/2 below 2^-27 for either sign). FMA is computed exactly in
128-bit integer arithmetic and rounded once.
Special values follow the C99 conventions: signed zeros are preserved, NaN propagates, and infinities and
poles are exact.

The kernels are generated as branch-free LLVM: every special case is a compare and a select, so a call costs
one straight-line block (only pow's repeated squaring and mod's long division loop). Rounding to integers
uses the LLVM floor, ceil, trunc, round and roundeven operations, which lower to single instructions where
the target has them.

</details>

<details class="doc-synopsis"><summary>Synopsis (79 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-acos-f64">acos-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">acos-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-acosh-f64">acosh-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">acosh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-asin-f64">asin-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asin-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-asinh-f64">asinh-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asinh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-atan-f64">atan-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-atan2-f64">atan2-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atan2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-atanh-f64">atanh-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atanh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-cbrt-f64">cbrt-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cbrt-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-ceil-f64">ceil-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ceil-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-copysign-f64">copysign-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">copysign-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-cos-f64">cos-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cos-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-cosh-f64">cosh-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cosh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-dim-f64">dim-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dim-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-e-f64">e-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-erf-f64">erf-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">erf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-erfc-f64">erfc-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">erfc-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-erfcinv-f64">erfcinv-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">erfcinv-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-erfinv-f64">erfinv-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">erfinv-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-exp-f64">exp-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">exp-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-exp2-f64">exp2-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">exp2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-expm1-f64">expm1-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">expm1-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-f32-bits">f32-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">f32-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-f32-from-bits">f32-from-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">f32-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-f64-bits">f64-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">f64-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-f64-from-bits">f64-from-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">f64-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-floor-f64">floor-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">floor-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-fma-f64">fma-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fma-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-frexp-f64">frexp-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">frexp-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-gamma-f64">gamma-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gamma-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-hypot-f64">hypot-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hypot-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-ilogb-f64">ilogb-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ilogb-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-inf-f64">inf-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">inf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-is-inf-f64">is-inf-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-inf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-nan-f64">is-nan-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-nan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-j0-f64">j0-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">j0-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-j1-f64">j1-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">j1-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-jn-f64">jn-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jn-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-ldexp-f64">ldexp-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ldexp-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-lgamma-f64">lgamma-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lgamma-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-ln10-f64">ln10-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ln10-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-ln2-f64">ln2-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ln2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-log-f64">log-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-log10-e-f64">log10-e-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log10-e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-log10-f64">log10-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log10-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-log1p-f64">log1p-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log1p-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-log2-e-f64">log2-e-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log2-e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-log2-f64">log2-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-logb-f64">logb-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">logb-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-max-f32">max-f32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-max-f64">max-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-max-of-f64">max-of-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-of-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-min-of-f64">min-of-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-of-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-mod-f64">mod-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mod-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-modf-f64">modf-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">modf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-nan-f64">nan-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-nextafter-f32">nextafter-f32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nextafter-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-nextafter-f64">nextafter-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nextafter-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-phi-f64">phi-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">phi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-pi-f64">pi-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-pow-f64">pow-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pow-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-pow10-f64">pow10-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pow10-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-remainder-f64">remainder-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">remainder-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-round-f64">round-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-round-to-even-f64">round-to-even-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round-to-even-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-sin-f64">sin-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sin-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-sincos-f64">sincos-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sincos-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-sinh-f64">sinh-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sinh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-smallest-f32">smallest-f32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smallest-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-smallest-f64">smallest-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smallest-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-sqrt-e-f64">sqrt-e-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sqrt-e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-sqrt-phi-f64">sqrt-phi-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sqrt-phi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-sqrt-pi-f64">sqrt-pi-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sqrt-pi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-sqrt2-f64">sqrt2-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sqrt2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-tan-f64">tan-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-tanh-f64">tanh-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tanh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-trunc-f64">trunc-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">trunc-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-y0-f64">y0-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">y0-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-y1-f64">y1-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">y1-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-yn-f64">yn-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">yn-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-acos-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">acos-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-acos-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3252" title="View source at line 3252">Source</a></span></div>
<div class="doc-body">

The arccosine, in [0, pi]; NaN outside [-1, 1].

</div>
</article>

<article class="doc-entry" id="v-acosh-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">acosh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-acosh-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L4031" title="View source at line 4031">Source</a></span></div>
<div class="doc-body">

The inverse hyperbolic cosine; NaN below 1.

</div>
</article>

<article class="doc-entry" id="v-asin-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asin-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-asin-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3184" title="View source at line 3184">Source</a></span></div>
<div class="doc-body">

The arcsine, in [-pi/2, pi/2]; NaN outside [-1, 1].

</div>
</article>

<article class="doc-entry" id="v-asinh-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asinh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-asinh-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3774" title="View source at line 3774">Source</a></span></div>
<div class="doc-body">

The inverse hyperbolic sine.

</div>
</article>

<article class="doc-entry" id="v-atan-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-atan-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3089" title="View source at line 3089">Source</a></span></div>
<div class="doc-body">

The arctangent, in [-pi/2, pi/2].

</div>
</article>

<article class="doc-entry" id="v-atan2-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atan2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-atan2-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3321" title="View source at line 3321">Source</a></span></div>
<div class="doc-body">

The angle of the point (x, y) from the positive x axis, in [-pi, pi], with signed-zero and infinite cases resolved by quadrant.

</div>
</article>

<article class="doc-entry" id="v-atanh-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atanh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-atanh-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L4285" title="View source at line 4285">Source</a></span></div>
<div class="doc-body">

The inverse hyperbolic tangent; NaN outside [-1, 1].

</div>
</article>

<article class="doc-entry" id="v-cbrt-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cbrt-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-cbrt-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L770" title="View source at line 770">Source</a></span></div>
<div class="doc-body">

The cube root, exact for perfect cubes of moderate size.

</div>
</article>

<article class="doc-entry" id="v-ceil-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ceil-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-ceil-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L216" title="View source at line 216">Source</a></span></div>
<div class="doc-body">

The least integer value not below x.

</div>
</article>

<article class="doc-entry" id="v-copysign-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">copysign-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-copysign-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

The magnitude of the first value with the sign of the second.

</div>
</article>

<article class="doc-entry" id="v-cos-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cos-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-cos-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L398" title="View source at line 398">Source</a></span></div>
<div class="doc-body">

The cosine.

</div>
</article>

<article class="doc-entry" id="v-cosh-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cosh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-cosh-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3581" title="View source at line 3581">Source</a></span></div>
<div class="doc-body">

The hyperbolic cosine.

</div>
</article>

<article class="doc-entry" id="v-dim-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dim-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-dim-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L295" title="View source at line 295">Source</a></span></div>
<div class="doc-body">

The positive difference: x - y when positive, otherwise zero.

</div>
</article>

<article class="doc-entry" id="v-e-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-e-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

e, the base of natural logarithms.

</div>
</article>

<article class="doc-entry" id="v-erf-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">erf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-erf-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L4471" title="View source at line 4471">Source</a></span></div>
<div class="doc-body">

The error function, 2/sqrt(pi) times the integral of exp(-t^2) from 0 to x.

</div>
</article>

<article class="doc-entry" id="v-erfc-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">erfc-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-erfc-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L4945" title="View source at line 4945">Source</a></span></div>
<div class="doc-body">

The complementary error function, 1 - erf(x), accurate where erf(x) is near 1.

</div>
</article>

<article class="doc-entry" id="v-erfcinv-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">erfcinv-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-erfcinv-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L5719" title="View source at line 5719">Source</a></span></div>
<div class="doc-body">

The inverse complementary error function: erfinv(1 - x).

</div>
</article>

<article class="doc-entry" id="v-erfinv-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">erfinv-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-erfinv-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L5424" title="View source at line 5424">Source</a></span></div>
<div class="doc-body">

The inverse error function: the y with erf(y) = x, for x in (-1, 1).

</div>
</article>

<article class="doc-entry" id="v-exp-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">exp-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-exp-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L858" title="View source at line 858">Source</a></span></div>
<div class="doc-body">

The natural exponential, e to the power x.

</div>
</article>

<article class="doc-entry" id="v-exp2-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">exp2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-exp2-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L931" title="View source at line 931">Source</a></span></div>
<div class="doc-body">

Two to the power x.

</div>
</article>

<article class="doc-entry" id="v-expm1-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">expm1-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-expm1-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L995" title="View source at line 995">Source</a></span></div>
<div class="doc-body">

e to the power x, minus one, accurate when x is near zero.

</div>
</article>

<article class="doc-entry" id="v-f32-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">f32-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-f32-bits" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

The bit pattern of a binary32 value.

</div>
</article>

<article class="doc-entry" id="v-f32-from-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">f32-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-f32-from-bits" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

The binary32 value with a bit pattern.

</div>
</article>

<article class="doc-entry" id="v-f64-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">f64-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-f64-bits" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

The bit pattern of a binary64 value.

</div>
</article>

<article class="doc-entry" id="v-f64-from-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">f64-from-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-f64-from-bits" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

The binary64 value with a bit pattern.

</div>
</article>

<article class="doc-entry" id="v-floor-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">floor-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-floor-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

The greatest integer value not above x (exact; signed zeros, infinities and NaN unchanged).

</div>
</article>

<article class="doc-entry" id="v-fma-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fma-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-fma-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L13173" title="View source at line 13173">Source</a></span></div>
<div class="doc-body">

x * y + z computed exactly and rounded once (round to nearest, ties to even).

</div>
</article>

<article class="doc-entry" id="v-frexp-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">frexp-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-frexp-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L245" title="View source at line 245">Source</a></span></div>
<div class="doc-body">

The fraction in [1/2, 1) and exponent with x = fraction * 2^exponent; zeros, infinities and NaN return
themselves with exponent 0.

</div>
</article>

<article class="doc-entry" id="v-gamma-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gamma-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-gamma-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L538" title="View source at line 538">Source</a></span></div>
<div class="doc-body">

The Gamma function; NaN at the negative integers and -Inf, signed infinities at the zeros.

</div>
</article>

<article class="doc-entry" id="v-hypot-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hypot-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-hypot-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L827" title="View source at line 827">Source</a></span></div>
<div class="doc-body">

The length of the hypotenuse, sqrt(p*p + q*q), without undue overflow or underflow.

</div>
</article>

<article class="doc-entry" id="v-ilogb-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ilogb-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-ilogb-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

The binary exponent of x as an integer: the least 32-bit integer for zero, the greatest for infinities
and NaN.

</div>
</article>

<article class="doc-entry" id="v-inf-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">inf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-inf-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L186" title="View source at line 186">Source</a></span></div>
<div class="doc-body">

Positive infinity for a sign of zero or more, negative infinity otherwise.

</div>
</article>

<article class="doc-entry" id="v-is-inf-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-inf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-inf-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

Whether a value is an infinity of a sign: positive for a sign above zero, negative below, either for zero.

</div>
</article>

<article class="doc-entry" id="v-is-nan-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-nan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-nan-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L194" title="View source at line 194">Source</a></span></div>
<div class="doc-body">

Whether a value is NaN.

</div>
</article>

<article class="doc-entry" id="v-j0-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">j0-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-j0-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L422" title="View source at line 422">Source</a></span></div>
<div class="doc-body">

The Bessel function of the first kind of order zero.

</div>
</article>

<article class="doc-entry" id="v-j1-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">j1-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-j1-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L426" title="View source at line 426">Source</a></span></div>
<div class="doc-body">

The Bessel function of the first kind of order one.

</div>
</article>

<article class="doc-entry" id="v-jn-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jn-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-jn-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L442" title="View source at line 442">Source</a></span></div>
<div class="doc-body">

The Bessel function of the first kind of integer order n.

</div>
</article>

<article class="doc-entry" id="v-ldexp-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ldexp-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-ldexp-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

fraction * 2^exponent, rounded once (gradual underflow, overflow to infinity).

</div>
</article>

<article class="doc-entry" id="v-lgamma-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lgamma-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-lgamma-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L414" title="View source at line 414">Source</a></span></div>
<div class="doc-body">

ln |Gamma(x)| and the sign of Gamma(x) (1 or -1); +Inf at zero and the non-positive integers.

</div>
</article>

<article class="doc-entry" id="v-ln10-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ln10-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-ln10-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L146" title="View source at line 146">Source</a></span></div>
<div class="doc-body">

The natural logarithm of 10.

</div>
</article>

<article class="doc-entry" id="v-ln2-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ln2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-ln2-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

The natural logarithm of 2.

</div>
</article>

<article class="doc-entry" id="v-log-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-log-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L1149" title="View source at line 1149">Source</a></span></div>
<div class="doc-body">

The natural logarithm.

</div>
</article>

<article class="doc-entry" id="v-log10-e-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log10-e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-log10-e-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

The decimal logarithm of e.

</div>
</article>

<article class="doc-entry" id="v-log10-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log10-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-log10-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L1226" title="View source at line 1226">Source</a></span></div>
<div class="doc-body">

The decimal logarithm.

</div>
</article>

<article class="doc-entry" id="v-log1p-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log1p-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-log1p-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L1413" title="View source at line 1413">Source</a></span></div>
<div class="doc-body">

The natural logarithm of 1 + x, accurate when x is near zero.

</div>
</article>

<article class="doc-entry" id="v-log2-e-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log2-e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-log2-e-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

The binary logarithm of e.

</div>
</article>

<article class="doc-entry" id="v-log2-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-log2-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L1304" title="View source at line 1304">Source</a></span></div>
<div class="doc-body">

The binary logarithm, exact for powers of two.

</div>
</article>

<article class="doc-entry" id="v-logb-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">logb-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-logb-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

The binary exponent of x as a value: negative infinity for zero, positive infinity for infinities.

</div>
</article>

<article class="doc-entry" id="v-max-f32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-max-f32" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

The largest finite binary32 value.

</div>
</article>

<article class="doc-entry" id="v-max-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-max-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L154" title="View source at line 154">Source</a></span></div>
<div class="doc-body">

The largest finite binary64 value.

</div>
</article>

<article class="doc-entry" id="v-max-of-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-of-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-max-of-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L299" title="View source at line 299">Source</a></span></div>
<div class="doc-body">

The larger value: positive infinity if either is, then NaN if either is; +0 beats -0.

</div>
</article>

<article class="doc-entry" id="v-min-of-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-of-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-min-of-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L307" title="View source at line 307">Source</a></span></div>
<div class="doc-body">

The smaller value: negative infinity if either is, then NaN if either is; -0 beats +0.

</div>
</article>

<article class="doc-entry" id="v-mod-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mod-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-mod-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

The remainder of x / y with the sign of x (truncated division); NaN when y is zero or x is infinite.

</div>
</article>

<article class="doc-entry" id="v-modf-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">modf-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-modf-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

The integer and fractional parts, both carrying the sign of x.

</div>
</article>

<article class="doc-entry" id="v-nan-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-nan-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

A quiet NaN.

</div>
</article>

<article class="doc-entry" id="v-nextafter-f32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nextafter-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-nextafter-f32" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

The next representable binary32 value after x toward y.

</div>
</article>

<article class="doc-entry" id="v-nextafter-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nextafter-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-nextafter-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L273" title="View source at line 273">Source</a></span></div>
<div class="doc-body">

The next representable value after x toward y.

</div>
</article>

<article class="doc-entry" id="v-phi-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">phi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-phi-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

The golden ratio.

</div>
</article>

<article class="doc-entry" id="v-pi-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-pi-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

pi, the nearest binary64 value.

</div>
</article>

<article class="doc-entry" id="v-pow-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pow-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-pow-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L355" title="View source at line 355">Source</a></span></div>
<div class="doc-body">

x to the power y, with the C99 special cases (pow(x, 0) = 1 and pow(1, y) = 1 even for NaN; negative
bases need integer exponents; signed zeros and infinities by parity).

</div>
</article>

<article class="doc-entry" id="v-pow10-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pow10-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-pow10-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L1969" title="View source at line 1969">Source</a></span></div>
<div class="doc-body">

Ten to an integer power, from exact tables.

</div>
</article>

<article class="doc-entry" id="v-remainder-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">remainder-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-remainder-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L337" title="View source at line 337">Source</a></span></div>
<div class="doc-body">

The IEEE remainder x - n*y with n the integer nearest x / y (halves to even).

</div>
</article>

<article class="doc-entry" id="v-round-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-round-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L230" title="View source at line 230">Source</a></span></div>
<div class="doc-body">

x rounded to the nearest integer value, halves away from zero.

</div>
</article>

<article class="doc-entry" id="v-round-to-even-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round-to-even-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-round-to-even-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L237" title="View source at line 237">Source</a></span></div>
<div class="doc-body">

x rounded to the nearest integer value, halves to even.

</div>
</article>

<article class="doc-entry" id="v-sin-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sin-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-sin-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L394" title="View source at line 394">Source</a></span></div>
<div class="doc-body">

The sine.

</div>
</article>

<article class="doc-entry" id="v-sincos-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sincos-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-sincos-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L406" title="View source at line 406">Source</a></span></div>
<div class="doc-body">

The sine and cosine together, sharing one argument reduction.

</div>
</article>

<article class="doc-entry" id="v-sinh-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sinh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-sinh-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3480" title="View source at line 3480">Source</a></span></div>
<div class="doc-body">

The hyperbolic sine.

</div>
</article>

<article class="doc-entry" id="v-smallest-f32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smallest-f32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-smallest-f32" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

The smallest positive (subnormal) binary32 value.

</div>
</article>

<article class="doc-entry" id="v-smallest-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smallest-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-smallest-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

The smallest positive (subnormal) binary64 value.

</div>
</article>

<article class="doc-entry" id="v-sqrt-e-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sqrt-e-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-sqrt-e-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

The square root of e.

</div>
</article>

<article class="doc-entry" id="v-sqrt-phi-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sqrt-phi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-sqrt-phi-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">

The square root of the golden ratio.

</div>
</article>

<article class="doc-entry" id="v-sqrt-pi-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sqrt-pi-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-sqrt-pi-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

The square root of pi.

</div>
</article>

<article class="doc-entry" id="v-sqrt2-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sqrt2-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-sqrt2-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

The square root of 2.

</div>
</article>

<article class="doc-entry" id="v-tan-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-tan-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L402" title="View source at line 402">Source</a></span></div>
<div class="doc-body">

The tangent.

</div>
</article>

<article class="doc-entry" id="v-tanh-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tanh-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-tanh-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L3662" title="View source at line 3662">Source</a></span></div>
<div class="doc-body">

The hyperbolic tangent.

</div>
</article>

<article class="doc-entry" id="v-trunc-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">trunc-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-trunc-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L223" title="View source at line 223">Source</a></span></div>
<div class="doc-body">

x rounded toward zero to an integer value.

</div>
</article>

<article class="doc-entry" id="v-y0-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">y0-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-y0-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L430" title="View source at line 430">Source</a></span></div>
<div class="doc-body">

The Bessel function of the second kind of order zero; NaN for negative x, -Inf at zero.

</div>
</article>

<article class="doc-entry" id="v-y1-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">y1-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-y1-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L434" title="View source at line 434">Source</a></span></div>
<div class="doc-body">

The Bessel function of the second kind of order one; NaN for negative x, -Inf at zero.

</div>
</article>

<article class="doc-entry" id="v-yn-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">yn-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-yn-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Float/source/#L517" title="View source at line 517">Source</a></span></div>
<div class="doc-body">

The Bessel function of the second kind of integer order n; NaN for negative x.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Float.cinch`](/packages/stdlib/Data.Float/source/). Source SHA-256: `ed1856b6e134453e2e8b1efa30c70515d533ce4d2b4a5d6a981fe4d7c9777d1a`.
