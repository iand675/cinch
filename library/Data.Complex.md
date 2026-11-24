# Data.Complex

Binary64 complex arithmetic, overflow-safe division, polar conversions, and full elementary, trigonometric, hyperbolic, and inverse functions over Data.Float.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Complex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Complex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Float](Data.Float.md), [Data.Machine](Data.Machine.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Complex](Data.Complex.md), [Data.Float](Data.Float.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-436f6d706c6578"></a>

## `Complex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Complex</span></code></pre>

A complex number with binary64 real and imaginary parts.

[Source](/reference-sources/Data.Complex.cinch) · line 37

# Values

<a id="value-616273"></a>

## `abs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">abs</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The modulus `|a + bi|`, computed overflow- and underflow-safely via `hypot-f64`.

[Source](/reference-sources/Data.Complex.cinch) · line 203

<a id="value-61636f73"></a>

## `acos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">acos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex inverse circular cosine.

[Source](/reference-sources/Data.Complex.cinch) · line 627

<a id="value-61636f7368"></a>

## `acosh`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">acosh</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex inverse hyperbolic cosine.

[Source](/reference-sources/Data.Complex.cinch) · line 633

<a id="value-616464"></a>

## `add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>))</code></pre>

Componentwise addition `(a + bi) + (c + di)`.

[Source](/reference-sources/Data.Complex.cinch) · line 124

<a id="value-617267"></a>

## `arg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">arg</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The four-quadrant argument of a complex number in `[-pi, pi]`.

[Source](/reference-sources/Data.Complex.cinch) · line 215

<a id="value-6173696e"></a>

## `asin`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asin</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex inverse circular sine.

[Source](/reference-sources/Data.Complex.cinch) · line 576

<a id="value-6173696e68"></a>

## `asinh`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asinh</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex inverse hyperbolic sine.

[Source](/reference-sources/Data.Complex.cinch) · line 603

<a id="value-6174616e"></a>

## `atan`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atan</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex inverse circular tangent.

[Source](/reference-sources/Data.Complex.cinch) · line 645

<a id="value-6174616e2d617070726f78"></a>

## `atan-approx`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atan-approx</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

Named arctangent approximation over `(-pi/2, pi/2)` retained for compatibility.

[Source](/reference-sources/Data.Complex.cinch) · line 680

<a id="value-6174616e68"></a>

## `atanh`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atanh</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex inverse hyperbolic tangent.

[Source](/reference-sources/Data.Complex.cinch) · line 674

<a id="value-636f6d706c6578"></a>

## `complex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">complex</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>))</code></pre>

Construct a complex number from its real and imaginary parts.

[Source](/reference-sources/Data.Complex.cinch) · line 42

<a id="value-636f6d706c65782d696e66"></a>

## `complex-inf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">complex-inf</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span></code></pre>

Complex infinity `(+Inf, +Inf)`.

[Source](/reference-sources/Data.Complex.cinch) · line 104

<a id="value-636f6d706c65782d6e616e"></a>

## `complex-nan`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">complex-nan</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span></code></pre>

Complex not-a-number `(NaN, NaN)`.

[Source](/reference-sources/Data.Complex.cinch) · line 108

<a id="value-636f6e6a"></a>

## `conj`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">conj</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

The complex conjugate `a - bi` (alias of `conjugate`).

[Source](/reference-sources/Data.Complex.cinch) · line 199

<a id="value-636f6e6a7567617465"></a>

## `conjugate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">conjugate</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

The complex conjugate `a - bi` (reflection across the real axis).

[Source](/reference-sources/Data.Complex.cinch) · line 195

<a id="value-636f73"></a>

## `cos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex circular cosine.

[Source](/reference-sources/Data.Complex.cinch) · line 392

<a id="value-636f7368"></a>

## `cosh`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cosh</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex hyperbolic cosine.

[Source](/reference-sources/Data.Complex.cinch) · line 416

<a id="value-636f74"></a>

## `cot`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cot</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex circular cotangent.

[Source](/reference-sources/Data.Complex.cinch) · line 566

<a id="value-646976"></a>

## `div`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>))</code></pre>

Overflow-safe complex division using Smith's scaling algorithm with C99
Annex G.5.1 infinity and zero recovery.

[Source](/reference-sources/Data.Complex.cinch) · line 160

<a id="value-6571"></a>

## `eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Structural equality: componentwise IEEE equality via `feq-f64`.

[Source](/reference-sources/Data.Complex.cinch) · line 207

<a id="value-657870"></a>

## `exp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">exp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex base-e exponential `e^z`.

[Source](/reference-sources/Data.Complex.cinch) · line 235

<a id="value-66726f6d2d7265616c"></a>

## `from-real`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-real</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Lift a real `F64` into `Complex` with zero imaginary part.

[Source](/reference-sources/Data.Complex.cinch) · line 56

<a id="value-696d61672d70617274"></a>

## `imag-part`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">imag-part</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The imaginary part of a complex number.

[Source](/reference-sources/Data.Complex.cinch) · line 51

<a id="value-69732d696e66"></a>

## `is-inf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-inf</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether either part is an infinity.

[Source](/reference-sources/Data.Complex.cinch) · line 112

<a id="value-69732d6e616e"></a>

## `is-nan`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-nan</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether either part is NaN and neither part is an infinity.

[Source](/reference-sources/Data.Complex.cinch) · line 116

<a id="value-6c6f67"></a>

## `log`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Principal natural logarithm `log(|z|) + i * phase(z)`.

[Source](/reference-sources/Data.Complex.cinch) · line 251

<a id="value-6c6f673130"></a>

## `log10`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log10</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Principal base-10 logarithm `log(z) / ln(10)`.

[Source](/reference-sources/Data.Complex.cinch) · line 255

<a id="value-6d756c"></a>

## `mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>))</code></pre>

Complex multiplication `(a + bi)(c + di) = (ac - bd) + (ad + bc)i`.

[Source](/reference-sources/Data.Complex.cinch) · line 134

<a id="value-6e6567"></a>

## `neg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">neg</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Componentwise additive inverse `-(a + bi) = -a - bi`.

[Source](/reference-sources/Data.Complex.cinch) · line 191

<a id="value-7068617365"></a>

## `phase`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">phase</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The phase (argument) of a complex number in `[-pi, pi]`, computed via `atan2-f64`.

[Source](/reference-sources/Data.Complex.cinch) · line 211

<a id="value-706f6c6172"></a>

## `polar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">polar</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>))</code></pre>

Polar representation `(r, theta)` such that `z = r * e^(i * theta)` with `theta` in `[-pi, pi]`.

[Source](/reference-sources/Data.Complex.cinch) · line 219

<a id="value-706f77"></a>

## `pow`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pow</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>))</code></pre>

Complex exponentiation `x^y`.

[Source](/reference-sources/Data.Complex.cinch) · line 261

<a id="value-7265616c2d70617274"></a>

## `real-part`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">real-part</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

The real part of a complex number.

[Source](/reference-sources/Data.Complex.cinch) · line 46

<a id="value-72656374"></a>

## `rect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rect</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>))</code></pre>

Construct a complex number from polar coordinates `(r, theta)`.

[Source](/reference-sources/Data.Complex.cinch) · line 223

<a id="value-73696e"></a>

## `sin`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sin</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex circular sine.

[Source](/reference-sources/Data.Complex.cinch) · line 344

<a id="value-73696e68"></a>

## `sinh`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sinh</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex hyperbolic sine.

[Source](/reference-sources/Data.Complex.cinch) · line 368

<a id="value-73717274"></a>

## `sqrt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sqrt</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Principal square root of `z`, with `real-part r >= 0` and `imag-part r` matching the sign of `imag-part z`.

[Source](/reference-sources/Data.Complex.cinch) · line 288

<a id="value-737562"></a>

## `sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>))</code></pre>

Componentwise subtraction `(a + bi) - (c + di)`.

[Source](/reference-sources/Data.Complex.cinch) · line 129

<a id="value-74616e"></a>

## `tan`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tan</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex circular tangent.

[Source](/reference-sources/Data.Complex.cinch) · line 531

<a id="value-74616e68"></a>

## `tanh`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tanh</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

Complex hyperbolic tangent.

[Source](/reference-sources/Data.Complex.cinch) · line 549

<a id="value-746f2d737472696e67"></a>

## `to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a complex number in `a+bi` or `a-bi` form.

[Source](/reference-sources/Data.Complex.cinch) · line 738

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Complex.Add_Complex` | `Algebra.Arithmetic.Add Data.Complex.Complex` | Implicit |
| `Data.Complex.Eq_Complex` | `Relation.Eq.Eq Data.Complex.Complex` | Implicit |
| `Data.Complex.Mul_Complex` | `Algebra.Arithmetic.Mul Data.Complex.Complex` | Implicit |
| `Data.Complex.Neg_Complex` | `Algebra.Arithmetic.Neg Data.Complex.Complex` | Implicit |
| `Data.Complex.Sub_Complex` | `Algebra.Arithmetic.Sub Data.Complex.Complex` | Implicit |

**`Data.Complex.Add_Complex`**: Componentwise addition for `Complex`.
**`Data.Complex.Eq_Complex`**: Componentwise IEEE equality for `Complex`.
**`Data.Complex.Mul_Complex`**: Complex multiplication for `Complex`.
**`Data.Complex.Neg_Complex`**: Componentwise negation for `Complex`.
**`Data.Complex.Sub_Complex`**: Componentwise subtraction for `Complex`.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Complex.cinch`. Source SHA-256: `a64d315deeb17f8b87b763ce97c337d6514f0826562151432828057fce558f4c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
