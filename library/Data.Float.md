# Data.Float

Binary64 mathematics: roundings, roots, exponentials, logarithms, powers, trigonometric, hyperbolic and special functions (erf, Gamma, Bessel) and FMA as branch-free kernels, bit-identical on every target.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Float`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Float.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Wide](Data.Wide.md).

Non-embedded source closure: [Data.Float](Data.Float.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

Every function is implemented in pure Cinch over Data.Machine and Data.Wide. Rounding to integers uses the
Data.Machine floor, ceil, trunc, round and round-to-even operations, which lower to single instructions
where the target has them.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61636f732d663634"></a>

## `acos-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">acos-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The arccosine, in [0, pi]; NaN outside [-1, 1].

[Source](/reference-sources/Data.Float.cinch) · line 890

<a id="value-61636f73682d663634"></a>

## `acosh-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">acosh-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The inverse hyperbolic cosine; NaN below 1.

[Source](/reference-sources/Data.Float.cinch) · line 970

<a id="value-6173696e2d663634"></a>

## `asin-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asin-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The arcsine, in [-pi/2, pi/2]; NaN outside [-1, 1].

[Source](/reference-sources/Data.Float.cinch) · line 875

<a id="value-6173696e682d663634"></a>

## `asinh-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asinh-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The inverse hyperbolic sine.

[Source](/reference-sources/Data.Float.cinch) · line 957

<a id="value-6174616e2d663634"></a>

## `atan-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atan-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The arctangent, in [-pi/2, pi/2].

[Source](/reference-sources/Data.Float.cinch) · line 868

<a id="value-6174616e322d663634"></a>

## `atan2-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atan2-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The angle of the point (x, y) from the positive x axis, in [-pi, pi], with signed-zero and infinite
cases resolved by quadrant.

[Source](/reference-sources/Data.Float.cinch) · line 895

<a id="value-6174616e682d663634"></a>

## `atanh-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atanh-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The inverse hyperbolic tangent; NaN outside [-1, 1].

[Source](/reference-sources/Data.Float.cinch) · line 981

<a id="value-636272742d663634"></a>

## `cbrt-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cbrt-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The cube root, exact for perfect cubes of moderate size.

[Source](/reference-sources/Data.Float.cinch) · line 377

<a id="value-6365696c2d663634"></a>

## `ceil-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ceil-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f70797369676e2d663634"></a>

## `copysign-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">copysign-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f732d663634"></a>

## `cos-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cos-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The cosine.

[Source](/reference-sources/Data.Float.cinch) · line 818

<a id="value-636f73682d663634"></a>

## `cosh-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cosh-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The hyperbolic cosine.

[Source](/reference-sources/Data.Float.cinch) · line 933

<a id="value-64696d2d663634"></a>

## `dim-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dim-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The positive difference: x - y when positive, otherwise zero.

[Source](/reference-sources/Data.Float.cinch) · line 294

<a id="value-652d663634"></a>

## `e-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">e-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

e, the base of natural logarithms.

[Source](/reference-sources/Data.Float.cinch) · line 78

<a id="value-6572662d663634"></a>

## `erf-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">erf-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The error function, 2/sqrt(pi) times the integral of exp(-t^2) from 0 to x.

[Source](/reference-sources/Data.Float.cinch) · line 1032

<a id="value-657266632d663634"></a>

## `erfc-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">erfc-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The complementary error function, 1 - erf(x), accurate where erf(x) is near 1.

[Source](/reference-sources/Data.Float.cinch) · line 1053

<a id="value-65726663696e762d663634"></a>

## `erfcinv-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">erfcinv-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The inverse complementary error function: erfinv(1 - x).

[Source](/reference-sources/Data.Float.cinch) · line 1108

<a id="value-657266696e762d663634"></a>

## `erfinv-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">erfinv-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The inverse error function: the y with erf(y) = x, for x in (-1, 1).

[Source](/reference-sources/Data.Float.cinch) · line 1079

<a id="value-6578702d663634"></a>

## `exp-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">exp-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The natural exponential, e to the power x.

[Source](/reference-sources/Data.Float.cinch) · line 437

<a id="value-657870322d663634"></a>

## `exp2-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">exp2-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

Two to the power x.

[Source](/reference-sources/Data.Float.cinch) · line 452

<a id="value-6578706d312d663634"></a>

## `expm1-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expm1-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

e to the power x, minus one, accurate when x is near zero.

[Source](/reference-sources/Data.Float.cinch) · line 494

<a id="value-6633322d62697473"></a>

## `f32-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f32-bits</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6633322d66726f6d2d62697473"></a>

## `f32-from-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f32-from-bits</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6636342d62697473"></a>

## `f64-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-bits</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

F64.

[Source](/reference-sources/Data.Machine.cinch) · line 574

<a id="value-6636342d66726f6d2d62697473"></a>

## `f64-from-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-from-bits</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666c6f6f722d663634"></a>

## `floor-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">floor-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666d612d663634"></a>

## `fma-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fma-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)))</code></pre>

x * y + z computed exactly and rounded once (round to nearest, ties to even).

[Source](/reference-sources/Data.Float.cinch) · line 1671

<a id="value-66726578702d663634"></a>

## `frexp-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">frexp-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-constructor">I64</span>))</code></pre>

The fraction in [1/2, 1) and exponent with x = fraction * 2^exponent; zeros, infinities and NaN return
themselves with exponent 0.

[Source](/reference-sources/Data.Float.cinch) · line 201

<a id="value-67616d6d612d663634"></a>

## `gamma-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gamma-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The Gamma function; NaN at the negative integers and -Inf, signed infinities at the zeros.

[Source](/reference-sources/Data.Float.cinch) · line 1242

<a id="value-6879706f742d663634"></a>

## `hypot-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hypot-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The length of the hypotenuse, sqrt(p*p + q*q), without undue overflow or underflow.

[Source](/reference-sources/Data.Float.cinch) · line 399

<a id="value-696c6f67622d663634"></a>

## `ilogb-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ilogb-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

The binary exponent of x as an integer: the least 32-bit integer for zero, the greatest for infinities
and NaN.

[Source](/reference-sources/Data.Float.cinch) · line 257

<a id="value-696e662d663634"></a>

## `inf-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">inf-f64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

Positive infinity for a sign of zero or more, negative infinity otherwise.

[Source](/reference-sources/Data.Float.cinch) · line 182

<a id="value-69732d696e662d663634"></a>

## `is-inf-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-inf-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Whether a value is an infinity of a sign: positive for a sign above zero, negative below, either for zero.

[Source](/reference-sources/Data.Float.cinch) · line 194

<a id="value-69732d6e616e2d663634"></a>

## `is-nan-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-nan-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a value is NaN.

[Source](/reference-sources/Data.Float.cinch) · line 190

<a id="value-6a302d663634"></a>

## `j0-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">j0-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The Bessel function of the first kind of order zero.

[Source](/reference-sources/Data.Float.cinch) · line 1393

<a id="value-6a312d663634"></a>

## `j1-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">j1-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The Bessel function of the first kind of order one.

[Source](/reference-sources/Data.Float.cinch) · line 1426

<a id="value-6a6e2d663634"></a>

## `jn-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jn-f64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The Bessel function of the first kind of integer order n.

[Source](/reference-sources/Data.Float.cinch) · line 1488

<a id="value-6c646578702d663634"></a>

## `ldexp-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ldexp-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

fraction * 2^exponent, rounded once (gradual underflow, overflow to infinity).

[Source](/reference-sources/Data.Float.cinch) · line 212

<a id="value-6c67616d6d612d663634"></a>

## `lgamma-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lgamma-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-constructor">I64</span>))</code></pre>

ln |Gamma(x)| and the sign of Gamma(x) (1 or -1); +Inf at zero and the non-positive integers.

[Source](/reference-sources/Data.Float.cinch) · line 1200

<a id="value-6c6e31302d663634"></a>

## `ln10-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ln10-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The natural logarithm of 10.

[Source](/reference-sources/Data.Float.cinch) · line 110

<a id="value-6c6e322d663634"></a>

## `ln2-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ln2-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The natural logarithm of 2.

[Source](/reference-sources/Data.Float.cinch) · line 102

<a id="value-6c6f672d663634"></a>

## `log-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The natural logarithm.

[Source](/reference-sources/Data.Float.cinch) · line 521

<a id="value-6c6f6731302d652d663634"></a>

## `log10-e-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log10-e-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The decimal logarithm of e.

[Source](/reference-sources/Data.Float.cinch) · line 114

<a id="value-6c6f6731302d663634"></a>

## `log10-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log10-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The decimal logarithm.

[Source](/reference-sources/Data.Float.cinch) · line 543

<a id="value-6c6f6731702d663634"></a>

## `log1p-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log1p-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The natural logarithm of 1 + x, accurate when x is near zero.

[Source](/reference-sources/Data.Float.cinch) · line 593

<a id="value-6c6f67322d652d663634"></a>

## `log2-e-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log2-e-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The binary logarithm of e.

[Source](/reference-sources/Data.Float.cinch) · line 106

<a id="value-6c6f67322d663634"></a>

## `log2-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log2-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The binary logarithm, exact for powers of two.

[Source](/reference-sources/Data.Float.cinch) · line 547

<a id="value-6c6f67622d663634"></a>

## `logb-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">logb-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The binary exponent of x as a value: negative infinity for zero, positive infinity for infinities.

[Source](/reference-sources/Data.Float.cinch) · line 264

<a id="value-6d61782d663332"></a>

## `max-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-f32</span> :: <span class="tok-constructor">F32</span></code></pre>

The largest finite binary32 value.

[Source](/reference-sources/Data.Float.cinch) · line 126

<a id="value-6d61782d663634"></a>

## `max-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The largest finite binary64 value.

[Source](/reference-sources/Data.Float.cinch) · line 118

<a id="value-6d61782d6f662d663634"></a>

## `max-of-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-of-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The larger value: positive infinity if either is, then NaN if either is; +0 beats -0.

[Source](/reference-sources/Data.Float.cinch) · line 298

<a id="value-6d696e2d6f662d663634"></a>

## `min-of-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-of-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The smaller value: negative infinity if either is, then NaN if either is; -0 beats +0.

[Source](/reference-sources/Data.Float.cinch) · line 306

<a id="value-6d6f642d663634"></a>

## `mod-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mod-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The remainder of x / y with the sign of x (truncated division); NaN when y is zero or x is infinite.

[Source](/reference-sources/Data.Float.cinch) · line 318

<a id="value-6d6f64662d663634"></a>

## `modf-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">modf-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>))</code></pre>

The integer and fractional parts, both carrying the sign of x.

[Source](/reference-sources/Data.Float.cinch) · line 238

<a id="value-6e616e2d663634"></a>

## `nan-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nan-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

A quiet NaN.

[Source](/reference-sources/Data.Float.cinch) · line 186

<a id="value-6e65787461667465722d663332"></a>

## `nextafter-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nextafter-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>))</code></pre>

The next representable binary32 value after x toward y.

[Source](/reference-sources/Data.Float.cinch) · line 281

<a id="value-6e65787461667465722d663634"></a>

## `nextafter-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nextafter-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The next representable value after x toward y.

[Source](/reference-sources/Data.Float.cinch) · line 272

<a id="value-7068692d663634"></a>

## `phi-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">phi-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The golden ratio.

[Source](/reference-sources/Data.Float.cinch) · line 82

<a id="value-70692d663634"></a>

## `pi-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pi-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

pi, the nearest binary64 value.

[Source](/reference-sources/Data.Float.cinch) · line 74

<a id="value-706f772d663634"></a>

## `pow-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pow-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

x to the power y, with the C99 special cases (pow(x, 0) = 1 and pow(1, y) = 1 even for NaN; negative
bases need integer exponents; signed zeros and infinities by parity).

[Source](/reference-sources/Data.Float.cinch) · line 619

<a id="value-706f7731302d663634"></a>

## `pow10-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pow10-f64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

Ten to an integer power, from exact tables.

[Source](/reference-sources/Data.Float.cinch) · line 688

<a id="value-72656d61696e6465722d663634"></a>

## `remainder-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">remainder-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The IEEE remainder x - n*y with n the integer nearest x / y (halves to even).

[Source](/reference-sources/Data.Float.cinch) · line 344

<a id="value-726f756e642d663634"></a>

## `round-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">round-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f756e642d746f2d6576656e2d663634"></a>

## `round-to-even-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">round-to-even-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73696e2d663634"></a>

## `sin-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sin-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The sine.

[Source](/reference-sources/Data.Float.cinch) · line 810

<a id="value-73696e636f732d663634"></a>

## `sincos-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sincos-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>))</code></pre>

The sine and cosine together, sharing one argument reduction.

[Source](/reference-sources/Data.Float.cinch) · line 832

<a id="value-73696e682d663634"></a>

## `sinh-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sinh-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The hyperbolic sine.

[Source](/reference-sources/Data.Float.cinch) · line 917

<a id="value-736d616c6c6573742d663332"></a>

## `smallest-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smallest-f32</span> :: <span class="tok-constructor">F32</span></code></pre>

The smallest positive (subnormal) binary32 value.

[Source](/reference-sources/Data.Float.cinch) · line 130

<a id="value-736d616c6c6573742d663634"></a>

## `smallest-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smallest-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The smallest positive (subnormal) binary64 value.

[Source](/reference-sources/Data.Float.cinch) · line 122

<a id="value-737172742d652d663634"></a>

## `sqrt-e-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sqrt-e-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The square root of e.

[Source](/reference-sources/Data.Float.cinch) · line 90

<a id="value-737172742d7068692d663634"></a>

## `sqrt-phi-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sqrt-phi-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The square root of the golden ratio.

[Source](/reference-sources/Data.Float.cinch) · line 98

<a id="value-737172742d70692d663634"></a>

## `sqrt-pi-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sqrt-pi-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The square root of pi.

[Source](/reference-sources/Data.Float.cinch) · line 94

<a id="value-73717274322d663634"></a>

## `sqrt2-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sqrt2-f64</span> :: <span class="tok-constructor">F64</span></code></pre>

The square root of 2.

[Source](/reference-sources/Data.Float.cinch) · line 86

<a id="value-74616e2d663634"></a>

## `tan-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tan-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The tangent.

[Source](/reference-sources/Data.Float.cinch) · line 824

<a id="value-74616e682d663634"></a>

## `tanh-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tanh-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The hyperbolic tangent.

[Source](/reference-sources/Data.Float.cinch) · line 941

<a id="value-7472756e632d663634"></a>

## `trunc-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trunc-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-79302d663634"></a>

## `y0-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">y0-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The Bessel function of the second kind of order zero; NaN for negative x, -Inf at zero.

[Source](/reference-sources/Data.Float.cinch) · line 1408

<a id="value-79312d663634"></a>

## `y1-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">y1-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The Bessel function of the second kind of order one; NaN for negative x, -Inf at zero.

[Source](/reference-sources/Data.Float.cinch) · line 1442

<a id="value-796e2d663634"></a>

## `yn-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">yn-f64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

The Bessel function of the second kind of integer order n; NaN for negative x.

[Source](/reference-sources/Data.Float.cinch) · line 1563

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Float.cinch`. Source SHA-256: `db71460088525283b38971012f3a13b55833060236fc4d14a7e9fb11e98e57e4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
